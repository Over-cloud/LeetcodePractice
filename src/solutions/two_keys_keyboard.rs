pub struct Solution;
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut factor = 1;
        for i in 1..n/2 {
            if n % i == 0 {
                factor = i;
            }
        }

        if n == 1 {
            return 0;
        }

        if factor == 1 {
            return n;
        }

        std::cmp::min(Solution::min_steps(factor) + (n / factor), (factor) + Solution::min_steps(n / factor))
    }
}
