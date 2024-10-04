use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut curr = 1;
        let mut result = Vec::new();
        while result.len() < n as usize {
            result.push(curr);
            if curr * 10 <= n { // try append 0
                curr = curr * 10;
            } else { // pop 9 then increase 1
                while curr == n || curr % 10 == 9 {
                    curr /= 10;
                }
                curr += 1;
            }
        }

        result
    }
}
