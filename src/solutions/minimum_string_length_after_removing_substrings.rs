use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new();
        for c in s.chars() {
            match (stack.last(), c) {
                (Some('A'), 'B') | (Some('C'), 'D') => {
                    stack.pop();
                },
                _ => {
                    stack.push(c);
                },
            }
        }
        
        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length_public_1() {
        let s = "ABFCACDB".to_string();
        let actual = Solution::min_length(s);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_min_length_public_2() {
        let s = "ACBBD".to_string();
        let actual = Solution::min_length(s);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}