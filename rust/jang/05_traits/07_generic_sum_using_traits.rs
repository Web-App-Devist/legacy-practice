// Define the Operation trait with the sum method
pub trait Operation {
    fn sum(&self, index: usize) -> i32;
}

// Implement the GeometricSeries struct
pub struct GeometricSeries {
    start: i32,
    ratio: i32,
}

impl GeometricSeries {
    pub fn new(start: i32, ratio: i32) -> Self {
        GeometricSeries { start, ratio }
    }

    pub fn iter(&self) -> GeometricSeriesIterator {
        GeometricSeriesIterator {
            current: self.start,
            ratio: self.ratio,
            count: 0,
        }
    }
}

// Implement the FibonacciSeries struct
pub struct FibonacciSeries;

impl FibonacciSeries {
    pub fn new() -> Self {
        FibonacciSeries
    }

    pub fn iter(&self) -> FibonacciSeriesIterator {
        FibonacciSeriesIterator { prev: 0, curr: 1 }
    }
}

// Implement the Operation trait for GeometricSeries
impl Operation for GeometricSeries {
    fn sum(&self, index: usize) -> i32 {
        self.iter().take(index).sum()
    }
}

// Implement the Operation trait for FibonacciSeries
impl Operation for FibonacciSeries {
    fn sum(&self, index: usize) -> i32 {
        self.iter().take(index).sum()
    }
}

// Iterator for GeometricSeries
pub struct GeometricSeriesIterator {
    current: i32,
    ratio: i32,
    count: usize,
}

impl Iterator for GeometricSeriesIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current *= self.ratio;
        self.count += 1;
        Some(value)
    }
}

// Iterator for FibonacciSeries
pub struct FibonacciSeriesIterator {
    prev: i32,
    curr: i32,
}

impl Iterator for FibonacciSeriesIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.prev;
        let new_curr = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = new_curr;
        Some(value)
    }
}

// Test the implementation
fn main() {
    let geo = GeometricSeries::new(1, 2); // Series 1, 2, 4, 8, ...
    let fib = FibonacciSeries::new(); // Series 0, 1, 1, 2, 3, 5, ...

    println!("Geometric Series sum up to index 5: {}", geo.sum(10)); // Output: 1 + 2 + 4 + 8 + 16 = 31
    println!("Fibonacci Series sum up to index 5: {}", fib.sum(10)); // Output: 0 + 1 + 1 + 2 + 3 = 7
}
