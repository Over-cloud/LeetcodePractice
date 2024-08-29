use crate::solutions::base_solution::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut row_map = HashMap::new();
        let mut col_map = HashMap::new();
        for (i, stone) in stones.iter().enumerate() {
            row_map.entry(stone[0]).or_insert_with(Vec::new).push(i);
            col_map.entry(stone[1]).or_insert_with(Vec::new).push(i);
        }

        let mut result = 0;
        let mut visited = vec![false; stones.len()];
        for i in 0..stones.len() {
            if visited[i] {
                continue;
            }

            let mut cnt = 0;
            let mut next = vec![i];
            while let Some(curr) = next.pop() {
                let row = stones[curr][0];
                if let Some(indices) = row_map.get(&row) {
                    for &idx in indices {
                        if visited[idx] == false {
                            visited[idx] = true;
                            next.push(idx);
                            cnt += 1;
                        }
                    }
                }
                let col = stones[curr][1];
                if let Some(indices) = col_map.get(&col) {
                    for &idx in indices {
                        if visited[idx] == false {
                            visited[idx] = true;
                            next.push(idx);
                            cnt += 1;
                        }
                    }
                }
            }
            result += cnt - 1;
        }
        
        result
    }
}
