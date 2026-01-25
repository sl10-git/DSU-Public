/// Extra Credit Version - 10,000 records
/// Import Contact struct from the hw1_dynamic_arrays user defined library.
use hw1_dynamic_arrays::Contact;

/// Standard library imports for error handling, file I/O, and timing.
use std::error::Error; // Error handling trait.
use std::fs::File; // File I/O operations trait.
use std::time::Instant; // Timing operations trait.

/// Main / Entry Point - Extra Credit Version
/// Return Type: Result<(), Box<dyn Error>>
/// Result type indicates success (()) or error (Box<dyn Error>).
/// Box<dyn Error> allows returning any error type that implements the Error trait.
fn main() -> Result<(), Box<dyn Error>> {   
    
    println!("=== Extra Credit Version: 10,000 Records ===\n");
    
    // Start runtime timing.
    let start = Instant::now();

    // Open the 10k CSV file.
    let file = File::open("data/10k-contacts.csv")?;
    
    // Create a CSV reader without headers.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // Create dynamic array (vec) to store contacts.
    // mut makes the vector mutable (can be changed).
    // Type annotation included for clarity; Rust could infer the type.
    let mut contacts: Vec<Contact> = Vec::new();

    // Read all contacts into the dynamic array. 
    // Use deserialize method to convert CSV rows into Contact structs.
    for result in reader.deserialize() {
        let contact: Contact = result?; // ? handles errors during deserialization.
        contacts.push(contact); // Push contact struct into the dynamic array.
    }

    // Print total number of contacts read; for testing / verification.
    println!("Total contacts read: {}", contacts.len());

    // Sort contacts by last name
    contacts.sort_by(|a, b| a.last_name.cmp(&b.last_name));  
    println!("\nSorted contacts by last name.");

    // Print every 1000th contact starting at index 999 (the 1000th contact)
    println!("\nPrinting every 1000th contact (starting at index 999):\n");
    
    let mut start_index = 999;
    let mut count = 0;
    while start_index < contacts.len() {
        count += 1;
        println!("Contact #{} (Index {}):", count, start_index);
        contacts[start_index].display();
        println!();
        start_index += 1000;
    }

    // End timing
    let duration = start.elapsed();
    println!("\nTotal execution time: {:?}", duration);

    // Return Ok result indicating successful execution.
    Ok(())
}
