// use std::collections::HashMap;
//
// fn main() {
//     let s = String::from("carrace");
//     let t = String::from("racecar");
//
//     if s.len() != t.len() {
//         println!("{}", false);
//     } else {
//         let mut s_hash_map = HashMap::new();
//         let mut t_hash_map = HashMap::new();
//
//         for i in s.chars() {
//             *s_hash_map.entry(i).or_insert(0) += 1;
//         }
//
//         for i in t.chars() {
//             *t_hash_map.entry(i).or_insert(0) += 1;
//         }
//
//         println!("{}", s_hash_map.eq(&t_hash_map));
//     }
// }

use std::collections::HashMap;

// Define the Solution struct
struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // If the lengths are not the same, they cannot be anagrams
        if s.len() != t.len() {
            return false;
        }

        // Create a HashMap to count the frequency of each character
        let mut map: HashMap<char, i64> = HashMap::new();

        // Traverse both strings in parallel
        for (char_s, char_t) in s.chars().zip(t.chars()) {
            // Increment the count for characters in `s`
            *map.entry(char_s).or_default() += 1;
            // Decrement the count for characters in `t`
            *map.entry(char_t).or_default() -= 1;
            println!("{:?}", map);
        }

        // Check that all counts are zero
        map.into_values().all(|count| count == 0)
    }
}

fn main() {
    let s1 = "anagram".to_string();
    let t1 = "nagaram".to_string();
    let s2 = "rat".to_string();
    let t2 = "car".to_string();

    println!(
        "Are '{}' and '{}' anagrams? {}",
        s1,
        t1,
        Solution::is_anagram(s1.clone(), t1.clone())
    ); // Should print true
    println!(
        "Are '{}' and '{}' anagrams? {}",
        s2,
        t2,
        Solution::is_anagram(s2.clone(), t2.clone())
    ); // Should print false
}
