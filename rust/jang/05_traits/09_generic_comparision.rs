fn get_greater<T: std::cmp::PartialEq + std::cmp::PartialOrd>(num1: T, num2: T) -> T {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}

fn main() {
    let num1: isize = 30;
    let num2 = 40;
    let prod = get_greater(num1, num2);
    println!("{}", prod);

    let num1: usize = 30;
    let num2 = 40;
    let prod = get_greater(num1, num2);
    println!("{}", prod);

    let num1 = 30.01;
    let num2 = 40.1;
    let prod = get_greater(num1, num2);
    println!("{}", prod);
}
