use std::collections::HashMap;
use std::fmt;

/// Represents a blockchain address
/// 
/// # Example
/// ```
/// use voting::Address;
/// let addr = Address(vec![1, 2, 3]);
/// assert_eq!(format!("{}", addr), "0x010203");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub Vec<u8>);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}

#[derive(Debug)]
pub enum VotingError {
    Unauthorized,
    InvalidOption,
    PollNotFound,
    PollEnded,
    AlreadyVoted,
    PollInactive
}

#[derive(Debug, Clone, PartialEq, Eq)]  // Remove Hash derive
pub struct Poll {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub votes: HashMap<Address, usize>,
    pub vote_counts: Vec<usize>, // Add this field
    pub end_time: u64,
    pub creator: Address,
    pub is_active: bool,
}

/// Represents the main voting contract that manages polls and votes
/// 
/// # Example
/// ```
/// use voting::{VotingContract, Address};
/// 
/// let mut contract = VotingContract::new();
/// let admin = Address(vec![1]);
/// let voter = Address(vec![2]);
/// 
/// // Add new admin
/// let default_admin = contract.admins[0].clone();
/// contract.add_admin(&default_admin, admin.clone()).unwrap();
/// 
/// // Create a new poll
/// let poll_id = contract.create_poll(
///     &admin,
///     "Best Programming Language".to_string(),
///     "Vote for your favorite".to_string(),
///     vec!["Rust".to_string(), "Go".to_string()],
///     86400
/// ).unwrap();
/// 
/// // Cast a vote
/// contract.cast_vote(&voter, poll_id, 0).unwrap();
/// 
/// // Get results
/// let results = contract.get_poll_results(poll_id).unwrap();
/// assert_eq!(results[0], ("Rust".to_string(), 1));
/// assert_eq!(results[1], ("Go".to_string(), 0));
/// ```
pub struct VotingContract {
    pub admins: Vec<Address>,
    polls: HashMap<u64, Poll>,
    next_poll_id: u64
}

impl VotingContract {
    /// Creates a new voting contract with a default admin
    /// 
    /// # Example
    /// ```
    /// use voting::VotingContract;
    /// let contract = VotingContract::new();
    /// assert_eq!(contract.admins.len(), 1);
    /// ```
    pub fn new() -> Self {
        Self {
            admins: vec![Address(vec![0])], // Default admin
            polls: HashMap::new(),
            next_poll_id: 1
        }
    }

    /// Adds a new admin to the contract
    /// 
    /// # Example
    /// ```
    /// use voting::{VotingContract, Address};
    /// let mut contract = VotingContract::new();
    /// let new_admin = Address(vec![1]);
    /// 
    /// let default_admin = contract.admins[0].clone();
    /// contract.add_admin(&default_admin, new_admin).unwrap();
    /// assert_eq!(contract.admins.len(), 2);
    /// ```
    pub fn add_admin(&mut self, caller: &Address, new_admin: Address) -> Result<(), VotingError> {
        if !self.admins.contains(caller) {
            return Err(VotingError::Unauthorized);
        }
        self.admins.push(new_admin);
        Ok(())
    }

    /// Creates a new poll with the given options
    /// 
    /// # Example
    /// ```
    /// use voting::{VotingContract, Address};
    /// let mut contract = VotingContract::new();
    /// let admin = contract.admins[0].clone();
    /// 
    /// let poll_id = contract.create_poll(
    ///     &admin,
    ///     "Favorite Color".to_string(),
    ///     "Vote for your favorite color".to_string(),
    ///     vec!["Blue".to_string(), "Red".to_string()],
    ///     86400
    /// ).unwrap();
    /// 
    /// let poll = contract.get_poll_details(poll_id).unwrap();
    /// assert_eq!(poll.options.len(), 2);
    /// ```
    pub fn create_poll(
        &mut self,
        caller: &Address,
        title: String,
        description: String,
        options: Vec<String>,
        duration: u64,
    ) -> Result<u64, VotingError> {
        if !self.admins.contains(caller) {
            return Err(VotingError::Unauthorized);
        }

        let poll_id = self.next_poll_id;
        self.next_poll_id += 1;

        let poll = Poll {
            title,
            description,
            options: options.clone(),
            votes: HashMap::new(),
            vote_counts: vec![0; options.len()], // Initialize vote counts
            end_time: duration,
            creator: caller.clone(),
            is_active: true
        };

        self.polls.insert(poll_id, poll);
        Ok(poll_id)
    }

    pub fn end_poll(&mut self, caller: &Address, poll_id: u64) -> Result<(), VotingError> {
        let poll = self.polls.get_mut(&poll_id).ok_or(VotingError::PollNotFound)?;
        
        if !self.admins.contains(caller) && &poll.creator != caller {
            return Err(VotingError::Unauthorized);
        }

        poll.is_active = false;
        Ok(())
    }

    pub fn cast_vote(&mut self, voter: &Address, poll_id: u64, option_idx: usize) -> Result<(), VotingError> {
        let poll = self.polls.get_mut(&poll_id).ok_or(VotingError::PollNotFound)?;
        
        if !poll.is_active {
            return Err(VotingError::PollInactive);
        }

        if poll.votes.contains_key(voter) {
            return Err(VotingError::AlreadyVoted);
        }

        if option_idx >= poll.options.len() {
            return Err(VotingError::InvalidOption);
        }

        poll.votes.insert(voter.clone(), option_idx);
        poll.vote_counts[option_idx] += 1; // Update vote count
        Ok(())
    }

    pub fn get_poll_results(&self, poll_id: u64) -> Result<Vec<(String, usize)>, VotingError> {
        let poll = self.polls.get(&poll_id).ok_or(VotingError::PollNotFound)?;
        Ok(poll.options.iter().cloned().zip(poll.vote_counts.iter().cloned()).collect())
    }

    pub fn get_active_polls(&self) -> Vec<(u64, &Poll)> {
        self.polls
            .iter()
            .filter(|(_, poll)| poll.is_active)
            .map(|(&id, poll)| (id, poll))
            .collect()
    }

    pub fn get_voter_participation(&self, voter: &Address) -> usize {
        self.polls
            .values()
            .filter(|poll| poll.votes.contains_key(voter))
            .count()
    }

    pub fn get_poll_details(&self, poll_id: u64) -> Result<&Poll, VotingError> {
        self.polls.get(&poll_id).ok_or(VotingError::PollNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_address(val: u8) -> Address {
        let mut bytes = vec![0; 20];
        bytes[0] = val;
        Address(bytes)
    }

    #[test]
    fn test_create_poll() {
        let mut contract = VotingContract::new();
        let admin = create_test_address(1);
        let default_admin = contract.admins[0].clone();
        contract.add_admin(&default_admin, admin.clone()).unwrap();

        let result = contract.create_poll(
            &admin,
            "Test Poll".to_string(),
            "Description".to_string(),
            vec!["Option A".to_string(), "Option B".to_string()],
            86400,
        ).unwrap();

        assert_eq!(result, 1);
        let poll = contract.get_poll_details(result).unwrap();
        assert_eq!(poll.options.len(), 2);
        assert_eq!(poll.vote_counts, vec![0, 0]);
        assert!(poll.is_active);
    }

    #[test]
    fn test_voting() {
        let mut contract = VotingContract::new();
        let admin = create_test_address(1);
        let voter = create_test_address(2);
        let default_admin = contract.admins[0].clone();
        contract.add_admin(&default_admin, admin.clone()).unwrap();

        let poll_id = contract.create_poll(
            &admin,
            "Test Poll".to_string(),
            "Description".to_string(),
            vec!["Option A".to_string(), "Option B".to_string()],
            86400,
        ).unwrap();

        // Test successful vote
        contract.cast_vote(&voter, poll_id, 0).unwrap();
        let poll = contract.get_poll_details(poll_id).unwrap();
        assert_eq!(poll.vote_counts[0], 1);
        assert_eq!(poll.vote_counts[1], 0);

        // Test duplicate vote
        assert!(matches!(
            contract.cast_vote(&voter, poll_id, 1),
            Err(VotingError::AlreadyVoted)
        ));
    }

    #[test]
    fn test_end_poll() {
        let mut contract = VotingContract::new();
        let admin = create_test_address(1);
        let voter = create_test_address(2);
        let default_admin = contract.admins[0].clone();
        contract.add_admin(&default_admin, admin.clone()).unwrap();

        let poll_id = contract.create_poll(
            &admin,
            "Test Poll".to_string(),
            "Description".to_string(),
            vec!["Option A".to_string()],
            86400,
        ).unwrap();

        contract.end_poll(&admin, poll_id).unwrap();
        assert!(matches!(
            contract.cast_vote(&voter, poll_id, 0),
            Err(VotingError::PollInactive)
        ));
    }

    #[test]
    fn test_unauthorized_actions() {
        let mut contract = VotingContract::new();
        let non_admin = create_test_address(2);

        // Test unauthorized poll creation
        assert!(matches!(
            contract.create_poll(
                &non_admin,
                "Test".to_string(),
                "Test".to_string(),
                vec!["Option".to_string()],
                86400
            ),
            Err(VotingError::Unauthorized)
        ));

        // Test unauthorized admin addition
        assert!(matches!(
            contract.add_admin(&non_admin, create_test_address(3)),
            Err(VotingError::Unauthorized)
        ));
    }

    #[test]
    fn test_get_active_polls() {
        let mut contract = VotingContract::new();
        let admin = create_test_address(1);
        let default_admin = contract.admins[0].clone();
        contract.add_admin(&default_admin, admin.clone()).unwrap();

        let poll1_id = contract.create_poll(
            &admin,
            "Poll 1".to_string(),
            "Description".to_string(),
            vec!["Option".to_string()],
            86400,
        ).unwrap();

        let poll2_id = contract.create_poll(
            &admin,
            "Poll 2".to_string(),
            "Description".to_string(),
            vec!["Option".to_string()],
            86400,
        ).unwrap();

        contract.end_poll(&admin, poll2_id).unwrap();
        let active_polls = contract.get_active_polls();
        assert_eq!(active_polls.len(), 1);
        assert_eq!(active_polls[0].0, poll1_id);
    }
}