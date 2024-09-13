fn main() {
    let mut num1 = 999;
    let mut num2 = 999;
    let mut result = 0;

    // Iterate through all pairs of 3-digit numbers
    while num1 >= 100 {
        while num2 >= 100 {
            let prod = num1 * num2;
            if is_palindrome(&prod.to_string()) && prod > result {
                result = prod;
            }
            num2 -= 1;
        }
        num1 -= 1;
        num2 = num1; // Reset num2 to the current value of num1
    }

    println!("{}", result);
}

fn is_palindrome(num: &str) -> bool {
    num.chars().eq(num.chars().rev())
}
