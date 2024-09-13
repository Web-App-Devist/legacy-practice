use std::collections::HashMap;

fn main() {
    let nums = [1, 2, 3, 4, 5];

    println!("{}", has_duplicate_character(nums.to_vec()));
}

fn has_duplicate_character(nums: Vec<i32>) -> bool {
    let mut hash_map = HashMap::new();

    for i in nums.iter() {
        if hash_map.contains_key(i) {
            return true;
        } else {
            hash_map.insert(i, 1);
        }
    }

    return false;
}
