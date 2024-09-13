fn multiply<T: std::ops::Mul<Output = T>>(num1: T, num2: T) -> T {
    num1 * num2
}

fn main() {
    let num1: isize = 30;
    let num2 = 40;
    let prod = multiply(num1, num2);
    println!("{}", prod);

    let num1: usize = 30;
    let num2 = 40;
    let prod = multiply(num1, num2);
    println!("{}", prod);

    let num1 = 30.01;
    let num2 = 40.1;
    let prod = multiply(num1, num2);
    println!("{}", prod);
}
