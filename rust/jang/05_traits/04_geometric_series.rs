// Define the GeometricSeries struct with fields: first_number, current_number, and ratio
struct GeometricSeries<T> {
    first_number: T,
    current_number: T,
    ratio: T,
}

// Implement the Iterator trait for GeometricSeries
impl<T> Iterator for GeometricSeries<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Item = T;

    // Define the next method to provide the next item in the series
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_number;
        // Calculate the next number in the series
        self.current_number = self.current_number * self.ratio;
        Some(result)
    }
}

// Implement the GeometricSeries struct methods
impl<T> GeometricSeries<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    // Method to create a new instance of GeometricSeries
    fn new(first_number: T, ratio: T) -> GeometricSeries<T> {
        GeometricSeries {
            first_number,
            current_number: first_number,
            ratio,
        }
    }
}

fn main() {
    let first_number = 2;
    let ratio = 3;
    let geometric_series = GeometricSeries::new(first_number, ratio);

    // Iterate over the next 11 numbers in the series and print them
    for number in geometric_series.take(11) {
        println!("{}", number);
    }

    let first_number = 2.3;
    let ratio = 3.2;
    let geometric_series = GeometricSeries::new(first_number, ratio);

    // Iterate over the next 11 numbers in the series and print them
    for number in geometric_series.take(11) {
        println!("{}", number);
    }
}
