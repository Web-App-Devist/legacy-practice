fn find_prime_factors(mut num: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Check for the smallest factor, 2
    while num % 2 == 0 {
        if !factors.contains(&2) {
            factors.push(2);
        }
        num /= 2;
    }

    // Check for odd factors from 3 to sqrt(num)
    let mut factor = 3;
    while factor * factor <= num {
        while num % factor == 0 {
            if !factors.contains(&factor) {
                factors.push(factor);
            }
            num /= factor;
        }
        factor += 2; // Move to the next odd number
    }

    // If num is still greater than 2, then num itself is a prime factor
    if num > 2 {
        factors.push(num);
    }

    factors.sort();
    factors
}

fn main() {
    let num: u64 = 600851475143;
    // let num = 13195;
    let factors = find_prime_factors(num);
    println!("{:?}", factors.iter().max().unwrap());
}
