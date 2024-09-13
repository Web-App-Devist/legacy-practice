use std::collections::HashSet;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Set {
    chars: Vec<char>,
}

impl Set {
    fn new(chars: Vec<char>) -> Self {
        Set { chars }
    }

    // Helper method to convert Vec<char> to HashSet<char> for easier subtraction
    fn to_hash_set(&self) -> HashSet<char> {
        self.chars.iter().cloned().collect()
    }
}

impl Sub for Set {
    type Output = Set;

    fn sub(self, other: Set) -> Set {
        let set1 = self.to_hash_set();
        let set2 = other.to_hash_set();

        let result: Vec<char> = set1.difference(&set2).cloned().collect();

        Set::new(result)
    }
}

fn main() {
    let set1 = Set::new(vec!['a', 'b', 'c', 'd']);
    let set2 = Set::new(vec!['b', 'd']);

    let result_set = set1 - set2;
    println!("{:?}", result_set); // Output will be Set { chars: ['a', 'c'] }
}
