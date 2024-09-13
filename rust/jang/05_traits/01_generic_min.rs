fn get_min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;
    let y = 10;
    let min_value = get_min(x, y);
    println!("Minimum value between {} and {} is {}", x, y, min_value);

    let s1 = "hello";
    let s2 = "world";
    let min_string = get_min(s1, s2);
    println!(
        "Minimum value between '{}' and '{}' is '{}'",
        s1, s2, min_string
    );
}
