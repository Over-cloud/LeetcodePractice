use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut max_len = 1;
        let mut curr_len = 1;
        
        for val in nums.into_iter().skip(1) {
            if val > max {
                max = val;
                max_len = 1;
                curr_len = 1;
            } else if val == max {
                curr_len += 1;
                max_len = max_len.max(curr_len);
            } else {
                if curr_len > max_len {
                    max_len = curr_len;
                }
                curr_len = 0;
            }
        }

        max_len
    }
}
