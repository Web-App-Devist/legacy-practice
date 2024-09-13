use std::fmt;

trait Drop {
    fn print_student_info(&self);
}

struct Student {
    name: String,
    age: u8,
    roll_number: String,
}

impl Drop for Student {
    fn print_student_info(&self) {
        println!(
            "Roll number {} has name {} with age {} and is a {}",
            self.roll_number,
            self.name,
            self.age,
            if self.age > 17 { "senior" } else { "junior" }
        )
    }
}

impl fmt::Debug for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Roll number {} has name {} with age {} and is a {}",
            self.roll_number,
            self.name,
            self.age,
            if self.age > 17 { "senior" } else { "junior" }
        )
    }
}

fn main() {
    let student = Student {
        age: 18,
        name: String::from("Tarun"),
        roll_number: "1234455".to_string(),
    };

    student.print_student_info();

    println!("Debugger: {:?}", student);
}
