#[derive(Debug, Clone, Copy, PartialEq)]
struct MyStruct {
    field1: i32,
    field2: f64,
}

fn main() {
    let instance1 = MyStruct {
        field1: 10,
        field2: 3.14,
    };

    // Cloning an instance
    let instance2 = instance1.clone(); // clone and copy(assignment) trait has been used

    // Printing debug information
    println!("Instance 1: {:?}", instance1); // debug trait has been used
    println!("Instance 2: {:?}", instance2); // debug trait has been used

    // Comparing instances for equality
    println!("Instance 1 equals Instance 2: {}", instance1 == instance2); // PartialEq trait has been used
}
