use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut pref_sum: Vec<i64> = vec![0; chalk.len()];
        pref_sum[0] = chalk[0] as i64;
        for i in 1..pref_sum.len() {
            pref_sum[i] = pref_sum[i - 1] + chalk[i] as i64;
        }

        let target = (k as i64) % pref_sum.last().unwrap();

        let mut left = 0 as i32;
        let mut right = pref_sum.len() as i32;
        while left <= right {
            let mid = ((left + right) / 2) as usize;
            if target < pref_sum[mid] {
                right = mid as i32 - 1;
            } else if target == pref_sum[mid] {
                return mid as i32 + 1;
            } else {
                left = mid as i32 + 1;
            }
        }

        return left;
    }
}
