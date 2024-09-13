fn main() {
    let limit: u64 = 4_000_000;
    let mut count: u64 = 0;
    let mut num1: u64 = 0;
    let mut num2: u64 = 1;

    while num1 < limit {
        let mut current_num: u64;
        current_num = num1 + num2;
        num1 = num2;
        num2 = current_num;
        if current_num % 2 == 0 {
            count += current_num;
        }
    }

    println!("{}", count);
}

// fn main() {
//     let limit = 10;
//     let mut a = 1;
//     let mut b = 2;
//     let mut even_sum = 0;
//
//     while a <= limit {
//         if a % 2 == 0 {
//             even_sum += a;
//         }
//         // Generate the next Fibonacci number
//         let temp = a;
//         a = b;
//         b = temp + b;
//
//         println!("{} {} {} {}", a, b, temp, even_sum);
//     }
//
//     println!(
//         "Sum of even-valued Fibonacci numbers up to {}: {}",
//         limit, even_sum
//     );
// }
