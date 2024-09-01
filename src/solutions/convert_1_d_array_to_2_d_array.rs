use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let row_max = m as usize;
        let col_max = n as usize;

        if row_max * col_max != original.len() {
            return Vec::new();
        }

        return original.chunks(col_max).map(|chunk| chunk.to_vec()).collect();
    }
}
