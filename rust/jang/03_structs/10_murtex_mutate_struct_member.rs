use std::sync::{Arc, Mutex};
use std::thread;

// Define a struct with a mutable member protected by Mutex
struct ImmutableStruct {
    mutable_member: Mutex<u32>,
}

impl ImmutableStruct {
    // Constructor method to create a new instance
    fn new(value: u32) -> Self {
        ImmutableStruct {
            mutable_member: Mutex::new(value),
        }
    }

    // Method to mutate the member (acquiring Mutex lock)
    fn mutate_member(&self, new_value: u32) {
        if let Ok(mut guard) = self.mutable_member.lock() {
            *guard = new_value;
        }
    }

    // Method to access the member (acquiring Mutex lock)
    fn get_member(&self) -> u32 {
        if let Ok(guard) = self.mutable_member.lock() {
            *guard
        } else {
            0 // Handle lock poisoning or other errors gracefully
        }
    }
}

fn main() {
    // Create an instance of ImmutableStruct wrapped in Arc for thread safety
    let my_struct = Arc::new(ImmutableStruct::new(5));

    println!("Initial value: {}", my_struct.get_member());

    // Clone Arc for use in multiple threads
    let my_struct_clone = Arc::clone(&my_struct);

    // Spawn a thread to mutate the member
    let handle = thread::spawn(move || {
        my_struct_clone.mutate_member(10);
    });

    // Wait for the thread to finish
    handle.join().unwrap();

    // Access the mutable member after mutation
    println!("Updated value: {}", my_struct.get_member());
}
