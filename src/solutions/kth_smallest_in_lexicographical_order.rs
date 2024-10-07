use crate::solutions::base_solution::Solution;
// TLE
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let k = k as i64;

        let mut curr = 1;
        let mut cnt = 1;
        while cnt < k {
            if curr * 10 <= n { // try append 0
                curr = curr * 10;
            } else { // pop 9 then increase 1
                while curr == n || curr % 10 == 9 {
                    curr /= 10;
                }
                curr += 1;
            }
            cnt += 1;
        }

        curr as i32
    }
}