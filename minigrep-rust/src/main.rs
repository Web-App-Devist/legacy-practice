use std::process;

use minigrep_rust::read_file;
use minigrep_rust::UserArgument;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let user_arguments = UserArgument::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let filtered_lines = read_file(&user_arguments.search, &user_arguments.file_name);

    println!("{:?}", filtered_lines);
}
