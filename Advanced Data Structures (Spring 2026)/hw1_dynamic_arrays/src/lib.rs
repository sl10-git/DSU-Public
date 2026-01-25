/// Import Deserialize trait from serde for automatic deserialization from CSV, rather than building the struct manually (with error checking, parsing, etc.).
use serde::Deserialize;

/// Derive attribute to auto-generate implementations for the specified traits.
/// Debug from Rust standard library for debugging output.
/// Deserialize from serde crate for automatic CSV-to-struct conversion.
/// Clone from Rust standard library to allow copying of struct instances.
#[derive(Debug, Deserialize, Clone)]

/// Define structure for contact information.
/// pub makes the struct accessible from other modules.
/// Fields match the CSV format: First name, Last name, Street, City, State, Zip, Phone, Email
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String, // String (not int) due to leading zeros in some zip codes (e.g., Maine 04101, Massachusetts 02101)
    pub phone: String,
    pub email: String,
}

/// Implementation Block - defines custom methods for Contact struct.
impl Contact {

    /// Public method to display contact information in a readable format.
    /// &self immutably borrows the instance without taking ownership (read-only access).
    pub fn display(&self) { 
        println!(
            "{} {}, {}, {}, {} {}, {}, {}", // {} placeholders get replaced with the values below
            self.first_name,
            self.last_name,
            self.street,
            self.city,
            self.state,
            self.zip,
            self.phone,
            self.email
        );
    }
}
