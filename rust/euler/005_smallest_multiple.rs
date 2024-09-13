fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut result = 1;
    for i in 1..=10 {
        result = lcm(result, i);
    }
    println!("The smallest positive number that is evenly divisible by all of the numbers from 1 to 20 is: {}", result);
}
