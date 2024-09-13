// Example 1 start
// struct Dimension {
//     length: i64,
//     breadth: i64,
// }
//
// struct DimentionTuple(i64, i64);
//
// fn rect_area(dimension: Dimension) -> i64 {
//     let Dimension { length, breadth } = dimension;
//
//     length * breadth
// }
//
// fn react_area_tuple(dimension_tuple: DimentionTuple) -> i64 {
//     let DimentionTuple(length, breadth) = dimension_tuple;
//     length * breadth
// }
//
// fn main() {
//     let rect_dimension = Dimension {
//         breadth: 200,
//         length: 300,
//     };
//
//     let area = rect_area(rect_dimension);
//
//     println!("{}", area);
//
//     let area_tuple = react_area_tuple(DimentionTuple(200, 300));
//     println!("{}", area_tuple);
// }
// Example 1 ends

// Example 2 start
// struct Point {
//     x: i64,
//     y: i64,
// }
//
// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
// }
//
// fn rect_area(dimension: Rectangle) -> i64 {
//     let Rectangle {
//         top_left: Point { x: x1, y: y1 },
//         bottom_right: Point { x: x2, y: y2 },
//     } = dimension;
//
//     let length = (x1 - x2).abs();
//     let breadth = (y1 - y2).abs();
//
//     length * breadth
// }
//
// fn main() {
//     let rectangle = Rectangle {
//         top_left: Point { x: 20, y: 30 },
//         bottom_right: Point { x: 70, y: 80 },
//     };
//
//     let area = rect_area(rectangle);
//
//     println!("{}", area);
// }
// Example 2 ends

// Example 3 starts

// use std::i64;
//
// #[derive(Debug)]
// struct Point {
//     x: i64,
//     y: i64,
// }
//
// #[derive(Debug)]
// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
// }
//
// fn square(point: Point, side_dimension: f32) -> Rectangle {
//     let (daigonal_length_x, daigonal_length_y) = (
//         f32::sqrt(side_dimension + (point.x as f32)) as i64,
//         f32::sqrt(side_dimension + (point.y as f32)) as i64,
//     );
//     Rectangle {
//         top_left: Point {
//             x: point.x,
//             y: point.y,
//         },
//         bottom_right: Point {
//             x: point.x + daigonal_length_x,
//             y: point.y + daigonal_length_y,
//         },
//     }
// }
//
// fn main() {
//     let test_square = square(Point { y: 40, x: 160 }, 9.0);
//
//     println!("{:?}", test_square)
// }

// Example 3 ends

// Example 4 start

// Example 4 ends
