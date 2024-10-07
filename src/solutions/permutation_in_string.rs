use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_len = s1.len();
        let s2_len = s2.len();
        if s1_len > s2_len {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        let mut arr1 = [0; 26];
        let mut arr2 = [0; 26];

        for &c in s1 {
            let offset = (c - b'a') as usize;
            arr1[offset] += 1;
        }

        for &c in &s2[..s1_len] {
            let offset = (c - b'a') as usize;
            arr2[offset] += 1;
        }

        if arr1 == arr2 {
            return true;
        }

        for window in s2.windows(s1_len + 1) {
            let prev = (window[0] as u8 - b'a') as usize;
            let curr = (window[s1_len] as u8 - b'a') as usize;
            arr2[prev] -= 1;
            arr2[curr] += 1;

            if arr1 == arr2 {
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
    fn test_check_inclusion_public_1() {
        let s1 = "abc".to_string();
        let s2 = "abc".to_string();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_check_inclusion_public_2() {
        let s1 = "abc".to_string();
        let s2 = "ababc".to_string();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_check_inclusion_public_3() {
        let s1 = "abc".to_string();
        let s2 = "cab".to_string();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_check_inclusion_public_4() {
        let s1 = "abc".to_string();
        let s2 = "bacab".to_string();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(actual, expected);
    }
}
