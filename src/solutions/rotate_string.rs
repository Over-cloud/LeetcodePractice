use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let s: Vec<_> = s.chars().collect();
        let goal: Vec<_> = goal.chars().collect();

        for start in 0..s.len() {
            let mut equal = true;
            for offset in 0..s.len() {
                let s_idx = (start + offset) % s.len();
                if s[s_idx] != goal[offset] {
                    equal = false;
                    break;
                }
            }
            if equal {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string_public_1() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();

        assert!(Solution::rotate_string(s, goal));
    }

    fn test_rotate_string_public_2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();

        assert!(!Solution::rotate_string(s, goal));
    }
}
