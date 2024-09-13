use std::cell::RefCell;

// Define a struct with a mutable member wrapped in RefCell
struct ImmutableStruct {
    mutable_member: RefCell<u32>,
}

impl ImmutableStruct {
    // Constructor method to create a new instance
    fn new(value: u32) -> Self {
        ImmutableStruct {
            mutable_member: RefCell::new(value),
        }
    }

    // Method to mutate the member (requires interior mutability)
    fn mutate_member(&self, new_value: u32) {
        *self.mutable_member.borrow_mut() = new_value;
    }

    // Method to access the member (borrowing)
    fn get_member(&self) -> u32 {
        *self.mutable_member.borrow()
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
