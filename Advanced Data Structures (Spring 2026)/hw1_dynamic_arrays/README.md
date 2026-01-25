# CSC 310 - Homework 1: Dynamic Arrays

**Student**: Stephen Leland  
**Language**: Rust  
**Date**: January 25, 2026

## Overview

Implementation of dynamic array operations using Rust's `Vec<T>` to read, sort, and process contact data from CSV files.

**Key Features**:

- Dynamic array (`Vec<Contact>`) that grows automatically
- CSV deserialization using serde
- In-place sorting by last name
- Direct array access for efficient indexed operations

## Project Structure

```
hw1_dynamic_arrays/
├── Cargo.toml              # Dependencies: csv, serde
├── README.md               # This documentation file
├── bin/                    # Pre-built binaries (Linux x64)
│   ├── hw1_dynamic_arrays  # Base version executable
│   └── extra_credit        # Extra credit executable
├── data/                   # CSV data files
│   ├── us-contacts.csv     # Base dataset (500 records)
│   └── 10k-contacts.csv    # Extra credit dataset (10,000 records)
└── src/
    ├── lib.rs              # Contact struct definition
    ├── main.rs             # Base implementation
    └── bin/
        └── extra_credit.rs # Extra credit (10k records)
```

## Running the Code

### Option 1: Run Pre-Built Binaries (Quick Start)

**For Linux/Mac users** - Run directly without installing Rust:

```bash
# Navigate to project directory
cd hw1_dynamic_arrays

# Base requirement (500 records)
./bin/hw1_dynamic_arrays

# Extra credit (10,000 records)
./bin/extra_credit
```

**Note**: Pre-built binaries are for Linux x64. Windows/other platforms should use Option 2.

### Option 2: Build from Source

**Prerequisites**:

- Rust toolchain installed (<https://rustup.rs/>)
- CSV data files in `data/` directory

**Steps**:

```bash
# 1. Navigate to project directory
cd hw1_dynamic_arrays

# 2. Ensure CSV files are present
ls data/us-contacts.csv data/10k-contacts.csv

# 3. Run base requirement (500 records)
cargo run

# 4. Run extra credit (10,000 records)
cargo run --bin extra_credit

# Optional: Build optimized release versions
cargo build --release
# Binaries will be in target/release/
```

## Implementation Details

### Dynamic Array

- **Type**: `Vec<Contact>` - Rust's growable array type
- **Growth**: Automatically resizes when capacity exceeded
- **Operations**:
  - `.push()` - Add contact to end
  - `[index]` - Direct access by position
  - `.sort_by()` - In-place sorting

### Contact Struct

```rust
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,      // String to preserve leading zeros
    pub phone: String,
    pub email: String,
}
```

### CSV Processing

1. Open CSV file without headers
2. Deserialize each row into Contact struct (automatic via serde)
3. Push each contact into Vec (dynamic growth)
4. Sort by last name using built-in sort
5. Print every 50th/1000th contact using direct index access

## Performance Results - Developer Test Runs

| Dataset | Records | Execution Time | Notes |
|---------|---------|----------------|-------|
| us-contacts.csv | 500 | ~1.3 ms | Base requirement |
| 10k-contacts.csv | 10,000 | ~23.4 ms | Extra credit |

## Key Code Sections

### Reading and Storing (Dynamic Array Growth)

```rust
let mut contacts: Vec<Contact> = Vec::new();

for result in reader.deserialize() {
    let contact: Contact = result?;
    contacts.push(contact);  // Vec grows automatically
}
```

### Sorting

```rust
contacts.sort_by(|a, b| a.last_name.cmp(&b.last_name));
```

### Direct Access

```rust
let mut index = 49;
while index < contacts.len() {
    contacts[index].display();  // O(1) direct access
    index += 50;
}
```

## Notes

- **Why Rust?**: I have been meaning to learn Rust, so chose this as a starting point, especially since it includes build-in dynamic array functionality via Vec and built-in, in-place array sorting.

- **AI / LLM Usage**: I used GitHub Copilot for Rust syntax guidance as I built this project. Prompts were about exploring and explaining Rust syntax basics and implementation. I started from the Rust documenation (<https://docs.rs/>) and the Rust Cookbook (<https://rust-lang-nursery.github.io/rust-cookbook/intro.html>). And from those starting points, I used more direct, nuanced prompts to build and explain each syntax block as I went. The comments reflect the additional, amplifying information I thought necessary to call our for myself and others.

- **Data Generation**: Used cobbl.io to generate 10k CSV records.
