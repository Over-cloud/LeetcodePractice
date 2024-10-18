use crate::solutions::base_solution::Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut s = Vec::new();

        let mut pq = BinaryHeap::new();
        if a > 0 {
            pq.push((a, Reverse('a')));
        }
        if b > 0 {
            pq.push((b, Reverse('b')));
        }
        if c > 0 {
            pq.push((c, Reverse('c')));
        }

        while let Some((cnt, Reverse(c))) = pq.pop() {
            let len = s.len();
            if len >= 2 && s[len - 1] == c && s[len - 2] == c {
                if let Some((next_cnt, Reverse(next_c))) = pq.pop() {
                    s.push(next_c);
                    s.push(c);
                    if next_cnt > 1 {
                        pq.push((next_cnt - 1, Reverse(next_c)));
                    }
                    if cnt > 1 {
                        pq.push((cnt - 1, Reverse(c)));
                    }
                } else {
                    break;
                }
            } else {
                s.push(c);
                if cnt > 1 {
                    pq.push((cnt - 1, Reverse(c)));
                }
            }
        }

        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_diverse_string_public_1() {
        assert_eq!(Solution::longest_diverse_string(0, 0, 0), "");
        assert_eq!(Solution::longest_diverse_string(1, 1, 1), "abc");
        assert_eq!(Solution::longest_diverse_string(2, 2, 2), "abcabc");
    }

    #[test]
    fn test_longest_diverse_string_public_2() {
        assert_eq!(Solution::longest_diverse_string(1, 0, 0), "a");
        assert_eq!(Solution::longest_diverse_string(2, 0, 0), "aa");
        assert_eq!(Solution::longest_diverse_string(3, 0, 0), "aa");
    }

    #[test]
    fn test_longest_diverse_string_public_3() {
        assert_eq!(Solution::longest_diverse_string(3, 2, 1), "aababc");
        assert_eq!(Solution::longest_diverse_string(4, 2, 1), "aabaabc");
        assert_eq!(Solution::longest_diverse_string(5, 2, 1), "aabaabac");
    }
}