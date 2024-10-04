use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut mask = 0;
        let mut map: Vec<i32> = vec![i32::MAX; 32]; //  a1 e1 i1 o1 u1
        let mut left_bound: Vec<i32> = vec![i32::MAX; 32];
        map[mask] = -1;

        let mut max_len = 0;
        let mut prev = 0;
        let mut prev_idx = 0;
        for (idx, ch) in s.chars().enumerate() {
            if ch != 'a' && ch != 'e' && ch != 'i' && ch != 'o' && ch != 'u' {
                continue;
            }

            if map[mask] != i32::MAX {
                if prev_idx == map[mask] {
                    max_len = max_len.max(idx as i32 - map[mask] - 1);
                } else {
                    max_len = max_len.max(idx as i32 - left_bound[mask] - 1);
                }
            }

            match ch {
                'a' => mask = mask ^ (1),
                'e' => mask = mask ^ (1 << 1),
                'i' => mask = mask ^ (1 << 2),
                'o' => mask = mask ^ (1 << 3),
                'u' => mask = mask ^ (1 << 4),
                _ => {},
            }

            if map[mask] == i32::MAX {
                map[mask] = idx as i32;
                left_bound[mask] = prev;
                prev = idx as i32 + 1;
                prev_idx = idx as i32;
            }
        }

        if map[mask] != i32::MAX {
            if prev_idx == map[mask] {
                max_len = max_len.max(s.len() as i32 - map[mask] - 1);
            } else {
                max_len = max_len.max(s.len() as i32 - left_bound[mask] - 1);
            }
        }

        max_len
    }
}
