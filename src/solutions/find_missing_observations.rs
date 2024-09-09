use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let sum = mean * (n + m);
        let partial_sum : i32 = rolls.iter().sum();
        let result_sum = sum - partial_sum;

        if result_sum < n || result_sum > n * 6 {
            return Vec::new();
        }

        let base = result_sum / n;
        let remainder = result_sum % n;
        let mut result = vec![base; n as usize];
        for i in 0..remainder as usize {
            result[i] += 1;
        }

        result
    }
}