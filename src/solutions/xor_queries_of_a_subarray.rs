use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let pref_sum = arr.into_iter().scan(0, |xor_sum, val| {
            *xor_sum = *xor_sum ^ val;
            Some(*xor_sum)
        }).collect::<Vec<_>>();
        
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            if left == 0 {
                result.push(pref_sum[right]);
            } else {
                result.push(pref_sum[left - 1] ^ pref_sum[right]);
            }
        }
        
        result
    }
}
