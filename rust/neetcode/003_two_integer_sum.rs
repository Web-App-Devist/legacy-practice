use std::collections::HashMap;

fn main() {
    let arr = [3, 4, 5, 6];
    let target = 8;
    let mut hash_map = HashMap::new();

    for (i, value) in arr.iter().enumerate() {
        if hash_map.contains_key(value) && 2 * value == target {
            return println!("{} {}", hash_map.get(value).unwrap(), i);
        } else {
            hash_map.insert(value, i);
        }
    }

    for value in arr {
        if hash_map.contains_key(&(target - value)) && value != target - value {
            println!(
                "{} {}",
                hash_map.get(&value).unwrap(),
                hash_map.get(&(target - value)).unwrap()
            );
            break;
        }
    }
}
