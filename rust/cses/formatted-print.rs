// Exmaple 1 starts
// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8,
// }
//
// fn main() {
//     let name = "Peter";
//     let age = 27;
//     let peter = Person { name, age };
//
//     // Pretty print
//     println!("{:#?}", peter);
// }
// Example 1 ends

// Example 2 start
// use std::fmt;
//
// #[derive(Debug)]
// struct Point2D {
//     num: f64,
//     x: &'static str,
//     y: String,
// }
//
// // Similarly, implement `Display` for `Point2D`.
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Customize so only `x` and `y` are denoted.
//         write!(f, "x: {}, y: {}, num: {}", self.x, self.y, self.num)
//     }
// }
//
// fn main() {
//     let point = Point2D {
//         x: "10.0",
//         y: "20.0".to_string(),
//         num: 10.0,
//     };
//
//     println!("{}", point);
//     println!("{:?}", point);
// }
// Example 2 ends

// Example 3 starts
// use std::fmt;
//
// #[derive(Debug)]
// struct ComplexNumber {
//     real_number: f64,
//     imaginary_number: f64,
// }
//
// impl fmt::Display for ComplexNumber {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} + {}i", self.real_number, self.imaginary_number)
//     }
// }
//
// fn main() {
//     let complex_number = ComplexNumber {
//         real_number: 3.3,
//         imaginary_number: 7.2,
//     };
//
//     println!("{}", complex_number);
//     println!("{:#?}", complex_number);
// }
// Example 3 ends

// Example 4 starts
// use std::fmt;
//
// struct RGB {
//     red: i32,
//     green: i32,
//     blue: i32,
// }
//
// impl fmt::Display for RGB {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let hex = (self.red * 65536) + (self.green * 256) + self.blue;
//
//         write!(
//             f,
//             "RGB ({}, {}, {}) 0x{:0>6X}",
//             self.red, self.green, self.blue, hex
//         )
//     }
// }
//
// fn main() {
//     let rgb_value1 = RGB {
//         red: 128,
//         green: 255,
//         blue: 90,
//     };
//     let rgb_value2 = RGB {
//         red: 0,
//         green: 3,
//         blue: 254,
//     };
//     let rgb_value3 = RGB {
//         red: 0,
//         green: 0,
//         blue: 0,
//     };
//
//     println!("{}", rgb_value1);
//     println!("{}", rgb_value2);
//     println!("{}", rgb_value3);
// }
// Example 4 ends
