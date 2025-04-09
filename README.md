# Votearch

A Rust-based implementation of a decentralized voting system that allows for creating and managing polls, casting votes, and tracking results.

## Features

- Create and manage multiple polls
- Admin management system
- Secure voting mechanism
- Poll results tracking
- Active poll monitoring
- Voter participation tracking

## Project Structure

```
voting/
├── src/
│   ├── main.rs      # Entry point and demo implementation
│   └── lib.rs       # Core contract implementation
├── Cargo.toml       # Project configuration and dependencies
└── deploy.sh        # Deployment script
```

## Core Components

### Address
Represents a blockchain address with hex encoding support.

### Poll
Represents a voting poll with:
- Title and description
- Multiple voting options
- Vote tracking
- End time
- Creator address
- Active status

### VotingContract
The main contract that manages:
- Admin management
- Poll creation and management
- Vote casting
- Results tracking
- Active poll monitoring

## Usage

### Creating a New Poll
```rust
let mut contract = VotingContract::new();
let admin = Address(vec![1]);
let poll_id = contract.create_poll(
    &admin,
    "Sample Poll".to_string(),
    "Description".to_string(),
    vec!["Option 1".to_string(), "Option 2".to_string()],
    3600 // Duration in seconds
)?;
```

### Casting a Vote
```rust
contract.cast_vote(&voter_address, poll_id, option_index)?;
```

### Getting Poll Results
```rust
let results = contract.get_poll_results(poll_id)?;
```

## Error Handling

The contract includes comprehensive error handling for various scenarios:
- Unauthorized access
- Invalid options
- Poll not found
- Poll ended
- Already voted
- Poll inactive

## Dependencies

- hex = "0.4.3"

## Building and Running

1. Ensure you have Rust installed
2. Clone the repository
3. Navigate to the project directory
4. Run `cargo build`
5. Run `cargo run` to execute the demo

## Testing

The project includes unit tests for all major functionality. Run tests using:
```bash
cargo test
```

## License

This project is open source and available under the MIT License. 
