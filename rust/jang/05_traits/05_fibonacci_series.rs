struct FibonacciSeries {
    current_num: u32,
    next_num: u32,
    total_sum: u32,
}

impl Iterator for FibonacciSeries {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_num + self.next_num;

        self.current_num = self.next_num;
        self.next_num = result;
        self.total_sum += result;
        Some((result, self.total_sum))
    }
}

impl FibonacciSeries {
    fn new() -> FibonacciSeries {
        FibonacciSeries {
            current_num: 0,
            next_num: 1,
            total_sum: 0,
        }
    }
}

fn main() {
    let fibonacci_series = FibonacciSeries::new();

    for (index, number) in fibonacci_series.take(11).enumerate() {
        println!(
            "{}th fibbonacci number is {} and the total sum is {}",
            index + 1,
            number.0,
            number.1
        );
    }
}
