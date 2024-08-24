use crate::solutions::base_solution::Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        if names.len() != heights.len() {
            return Vec::new();
        }

        let mut map = BTreeMap::new();
        for (name, height) in names.into_iter().zip(heights) {
            map.insert(height, name);
        }

        map.into_iter().rev().map(|(_, name)| name).collect()
    }
}