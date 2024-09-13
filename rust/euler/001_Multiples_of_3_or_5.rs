fn main() {
    let range = 1000;
    let mut count = 0;

    for i in 1..range {
        if i % 3 == 0 || i % 5 == 0 {
            count += i;
        }
    }

    println!("{}", count);
}
