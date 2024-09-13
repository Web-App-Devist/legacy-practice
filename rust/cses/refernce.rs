fn main() {
    let mut x = 10;
    {
        let mut_x = &mut x;
        *mut_x += 1;
    }

    println!("{}", x)
}
