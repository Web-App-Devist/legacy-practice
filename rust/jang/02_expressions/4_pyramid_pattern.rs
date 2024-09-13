use std::io;

fn main() {
    let mut number_of_columns = String::new();
    io::stdin().read_line(&mut number_of_columns).unwrap();
    let number_of_columns: i32 = number_of_columns.trim().parse().unwrap();

    let number_of_rows = 2 * number_of_columns - 1;

    let mid_point = number_of_rows / 2;

    for j in 0..number_of_columns {
        let mut count = 0;
        for i in 0..number_of_rows {
            if (mid_point - i).abs() > j {
                print!("   ");
            } else {
                if count < j + 1 && i <= mid_point {
                    count += 1;
                } else {
                    count -= 1;
                }
                print!(" {} ", count);
            }
        }
        println!("");
    }
}
