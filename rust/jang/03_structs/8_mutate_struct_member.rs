use std::cell::Cell;

// Define a struct with a mutable member wrapped in Cell
struct ImmutableStruct {
    mutable_member: Cell<u32>,
}

impl ImmutableStruct {
    // Constructor method to create a new instance
    fn new(value: u32) -> Self {
        ImmutableStruct {
            mutable_member: Cell::new(value),
        }
    }

    // Method to mutate the member (using Cell's set method)
    fn mutate_member(&self, new_value: u32) {
        self.mutable_member.set(new_value);
    }

    // Method to access the member (using Cell's get method)
    fn get_member(&self) -> u32 {
        self.mutable_member.get()
    }
}

fn main() {
    // Create an instance of ImmutableStruct
    let my_struct = ImmutableStruct::new(5);

    // Access the mutable member (borrowing)
    println!("Initial value: {}", my_struct.get_member());

    // Mutate the mutable member
    my_struct.mutate_member(10);

    // Access the mutable member again
    println!("Updated value: {}", my_struct.get_member());
}
