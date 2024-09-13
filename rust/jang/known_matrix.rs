// #[derive(Debug, Clone, Copy)]
// struct Point {
//     x: usize,
//     y: usize,
// }
//
// #[derive(Debug)]
// struct Matrix {
//     data: Vec<Vec<i32>>,
// }
//
// impl Matrix {
//     fn new(rows: usize, cols: usize, initial_value: i32) -> Self {
//         let data = vec![vec![initial_value; cols]; rows];
//         Matrix { data }
//     }
//
//     fn rows(&self) -> usize {
//         self.data.len()
//     }
//
//     fn cols(&self) -> usize {
//         self.data[0].len()
//     }
//
//     fn get(&self, point: Point) -> Option<i32> {
//         if point.y < self.rows() && point.x < self.cols() {
//             Some(self.data[point.y][point.x])
//         } else {
//             None
//         }
//     }
//
//     // Create a new iterator for the matrix
//     fn iter(&self) -> MatrixIterator {
//         MatrixIterator {
//             matrix: self,
//             current_x: 0,
//             current_y: 0,
//         }
//     }
// }
//
// // Define an iterator for the Matrix
// #[derive(Debug)]
// struct MatrixIterator<'a> {
//     matrix: &'a Matrix,
//     current_x: usize,
//     current_y: usize,
// }
//
// impl<'a> Iterator for MatrixIterator<'a> {
//     type Item = (Point, i32);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current_y >= self.matrix.rows() {
//             return None; // End of iteration
//         }
//
//         let point = Point {
//             x: self.current_x,
//             y: self.current_y,
//         };
//
//         let value = self.matrix.get(point)?;
//
//         // Move to the next position
//         self.current_x += 1;
//         if self.current_x >= self.matrix.cols() {
//             self.current_x = 0;
//             self.current_y += 1;
//         }
//
//         Some((point, value))
//     }
// }
//
// fn main() {
//     let mut matrix = Matrix::new(3, 3, 0);
//     matrix.data[1][1] = 5;
//     matrix.data[2][2] = 10;
//
//     let mut iter = matrix.iter();
//
//     while let Some((point, value)) = iter.next() {
//         println!("Point ({}, {}): {}", point.x, point.y, value);
//     }
//
//     println!("{:?}", iter.take(3))
// }

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Matrix {
    rows: i16,
    columns: i16,
    data: Vec<Vec<i32>>,
    current_row: i16,
    current_col: i16,
}

impl Matrix {
    fn new(rows: i16, columns: i16, initial_value: i32) -> Matrix {
        Matrix {
            rows,
            columns,
            data: vec![vec![initial_value; columns as usize]; rows as usize],
            current_row: 0,
            current_col: 0,
        }
    }
}

impl Iterator for Matrix {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.rows {
            return None; // End of iteration
        }

        let value = self.data[self.current_row as usize][self.current_col as usize];

        // Move to the next position
        self.current_col += 1;
        if self.current_col >= self.columns {
            self.current_col = 0;
            self.current_row += 1;
        }

        Some(value)
    }
}

fn main() {
    let point = Point::new(10, 20);

    let mut matrix = Matrix::new(2, 3, 10); // Changed columns to 3 for better demonstration

    matrix.data[0][1] = 5; // Changing some values for demonstration
    matrix.data[1][2] = 15;

    println!("{:?} {:?}", point, matrix);

    for value in matrix.take(3) {
        println!("Matrix value: {}", value);
    }

    // println!("{:?}", matrix.take(1))
}
