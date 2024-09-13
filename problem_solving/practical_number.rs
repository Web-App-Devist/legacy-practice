fn main() {
    practical(100);
}

fn practical(num: i64) {
    let mut factor_arr = vec![];
    for i in 1..(num / 2) + 1 {
        if num % i == 0 {
            factor_arr.push(i)
        }
    }
    factor_arr.push(num);

    println!("{:?}", factor_arr);
}
