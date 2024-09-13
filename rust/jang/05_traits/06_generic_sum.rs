// Define the GeometricSeries struct
struct GeometricSeries {
    start: i32,
    ratio: i32,
    current: i32,
}

// Implement Iterator for GeometricSeries
impl Iterator for GeometricSeries {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current *= self.ratio;
        Some(result)
    }
}

// Define the FibonacciSeries struct
struct FibonacciSeries {
    prev: i32,
    curr: i32,
}

// Implement Iterator for FibonacciSeries
impl Iterator for FibonacciSeries {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.prev;
        let new_curr = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = new_curr;
        Some(result)
    }
}

// Define the generic sum function
fn sum<T: Iterator<Item = i32>>(iter: T, index: usize) -> i32 {
    iter.take(index + 1).sum()
}

fn main() {
    // Create a GeometricSeries iterator
    let geo_series = GeometricSeries {
        start: 1,
        ratio: 2,
        current: 1,
    };

    // Create a FibonacciSeries iterator
    let fib_series = FibonacciSeries { prev: 0, curr: 1 };

    // Calculate the sum up to index 4 for GeometricSeries
    let geo_sum = sum(geo_series, 4);
    println!("Sum of GeometricSeries up to index 4: {}", geo_sum);

    // Calculate the sum up to index 5 for FibonacciSeries
    let fib_sum = sum(fib_series, 5);
    println!("Sum of FibonacciSeries up to index 5: {}", fib_sum);
}
