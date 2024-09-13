# Process 1

- create main function to accept arguments
- in the main function, load a file and read the content

# Process 2

- create function to parse the input
- create a function to accept a path of a file and return the content of the file

# Process 3

- convert the parse function into struct and implementation
- handle the error case by using `Result`, `Ok` and `Err` functions
- handle this error in the main function

# Process 4

- extract out the code for loading file and reading its content
- handle error case(try using `if let Err`)

---XXXXXX---

# Process 5

- segregate the code into different file (lib.rs)
- write tests
