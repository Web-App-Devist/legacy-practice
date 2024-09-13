// fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
//     let mut is_prime = vec![true; limit + 1];
//     let mut primes = Vec::new();
//
//     for p in 2..=limit {
//         if is_prime[p] {
//             primes.push(p);
//             let mut multiple = p * p;
//             while multiple <= limit {
//                 is_prime[multiple] = false;
//                 multiple += p;
//             }
//         }
//     }
//
//     primes
// }
//
// fn find_nth_prime(n: usize) -> usize {
//     // Estimate an upper limit for the sieve
//     let limit = if n < 6 {
//         15
//     } else {
//         (n as f64 * (n as f64).ln() + (n as f64).ln()).ceil() as usize
//     };
//
//     let mut primes = sieve_of_eratosthenes(limit);
//
//     while primes.len() < n {
//         // Increase the limit and re-run the sieve
//         let new_limit = limit * 2;
//         primes = sieve_of_eratosthenes(new_limit);
//     }
//
//     primes[n - 1]
// }
//
// fn main() {
//     // Find the 10001st prime
//     let nth_prime = find_nth_prime(10001);
//     println!("The 10001st prime number is {}", nth_prime);
// }

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    // 0 and 1 are not prime numbers
    if limit >= 0 {
        is_prime[0] = false;
    }
    if limit >= 1 {
        is_prime[1] = false;
    }

    for p in 2..=limit {
        if is_prime[p] {
            primes.push(p);
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
    }

    primes
}

fn find_nth_prime(n: usize) -> usize {
    // Initial estimate of the upper bound for the sieve
    let mut limit = if n < 6 {
        15
    } else {
        (n as f64 * (n as f64).ln() * 1.2).ceil() as usize
    };

    loop {
        let primes = sieve_of_eratosthenes(limit);
        if primes.len() >= n {
            return primes[n - 1];
        }
        limit *= 2; // Double the limit and try again
    }
}

fn main() {
    // Find the 10001st prime
    let nth_prime = find_nth_prime(10001);
    println!("The 10001st prime number is {}", nth_prime);
}
