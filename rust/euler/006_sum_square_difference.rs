// fn main() {
//     let num = 100;
//     let mut result = 0;
//
//     for i in 1..num + 1 {
//         let mut sum = 0;
//         for j in i + 1..num + 1 {
//             sum += j;
//         }
//         result += i * sum;
//     }
//
//     println!("{}", result * 2)
// }

fn main() {
    let num = 100;

    let square_sum = num * (num + 1) * (2 * num + 1) / 6;
    let sum_square = (num * (num + 1) / 2) * (num * (num + 1) / 2);

    println!("{}", sum_square - square_sum);
}
