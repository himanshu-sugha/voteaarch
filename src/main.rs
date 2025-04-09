use std::time::{SystemTime, UNIX_EPOCH};
use voting::{Address, VotingContract};

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    let contract = VotingContract::new();
    let _admin = Address(vec![0; 20]);
    
    println!("Voting Contract Demo");
    println!("Default admin: {}", contract.admins[0]);
}

