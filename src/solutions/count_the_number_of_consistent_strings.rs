use crate::solutions::base_solution::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed_set: HashSet<_> = allowed.chars().collect();

        words.iter().filter(|word| Self::is_string_consistent(&allowed_set, word)).count() as i32
    }

    #[inline]
    fn is_string_consistent(allowed: &HashSet<char>, word: &str) -> bool {
        word.chars().all(|c| allowed.contains(&c))
    }
}
