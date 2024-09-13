use std::fs::read_to_string;

pub struct UserArgument {
    pub search: String,
    pub file_name: String,
}

impl UserArgument {
    pub fn new(args: &[String]) -> Result<UserArgument, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        if args.len() > 3 {
            return Err("Too many arguments");
        }

        let search = args[1].clone();
        let file_name = args[2].clone();

        Ok(UserArgument { search, file_name })
    }
}

pub fn read_file(search: &str, file_name: &str) -> Vec<String> {
    let file_content = read_to_string(file_name).expect("Failed to read file");

    let search = search.to_lowercase();
    let mut filtered_lines = Vec::new();

    for line in file_content.lines() {
        if line.to_lowercase().contains(&search.to_lowercase()) {
            filtered_lines.push(line.to_string());
        }
    }

    filtered_lines
}
