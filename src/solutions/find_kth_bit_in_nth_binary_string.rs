use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut i = 1;
        let mut len = 1;
        while i < n {
            len = len * 2 + 1;
            i += 1;
        }

        if Self::find_kth_bit_aux(len, k as u32 - 1) {
            '1'
        } else {
            '0'
        }
    }

    fn find_kth_bit_aux(len: u32, k: u32) -> bool {
        if len == 1 {
            return false;
        }

        let mid = len / 2;
        if k == mid {
            return true;
        } else if k < mid {
            return Self::find_kth_bit_aux(mid, k);
        } else {
            return !Self::find_kth_bit_aux(mid, len - k - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_bit_basic_1() {
        assert_eq!(Solution::find_kth_bit(1, 1), '0');
        assert_eq!(Solution::find_kth_bit(2, 1), '0');
        assert_eq!(Solution::find_kth_bit(2, 2), '1');
        assert_eq!(Solution::find_kth_bit(2, 3), '1');
    }

    #[test]
    fn test_find_kth_bit_public_1() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
