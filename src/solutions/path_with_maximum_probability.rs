use crate::solutions::base_solution::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        if edges.len() != succ_prob.len() {
            return 0.0;
        }

        let mut map = HashMap::new();
        for i in 0..edges.len() {
            let start = edges[i][0] as usize;
            let end = edges[i][1] as usize;
            let prob = succ_prob[i];
            map.entry(start).or_insert_with(Vec::new).push((end, prob));
            map.entry(end).or_insert_with(Vec::new).push((start, prob));
        }

        let mut prob_vec = vec![-1.0; n as usize];
        prob_vec[start_node as usize] = 1.0;
        let mut stack = vec![(start_node as usize, 1.0)];

        let delta : f64 = 1e-5;
        while let Some((curr, acc_prob)) = stack.pop() {
            if let Some(prob_list) = map.get(&curr) {
                for (next, edge_prob) in prob_list {
                    let next_prob = acc_prob * edge_prob;
                    if prob_vec[*next] < 0.0 || next_prob > prob_vec[*next] + delta {
                        prob_vec[*next] = next_prob;
                        stack.push((*next, next_prob));
                    }
                }
            }
        }

        if prob_vec[end_node as usize] < 0.0 {
            0.0
        } else {
            prob_vec[end_node as usize]
        }
    }
}
