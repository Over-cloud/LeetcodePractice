use crate::solutions::base_solution::Solution;
use crate::utils::linked_list::ListNode;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let n_row = m as usize;
        let n_col = n as usize;
        let mut result = vec![vec![-1; n_col]; n_row];

        let mut curr_row = 0isize;
        let mut curr_col = 0isize;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0),];
        let mut dir_idx = 0;

        let mut curr_node = head.as_ref();
        while let Some(node) = curr_node {
            result[curr_row as usize][curr_col as usize] = node.val;

            let next_row = curr_row + directions[dir_idx].0;
            let next_col = curr_col + directions[dir_idx].1;

            if next_row < 0 || next_row >= n_row as isize ||
               next_col < 0 || next_col >= n_col as isize||
               result[next_row as usize][next_col as usize] != -1 {
                dir_idx = (dir_idx + 1) % 4;
                curr_row = curr_row + directions[dir_idx].0;
                curr_col = curr_col + directions[dir_idx].1;
            } else {
                curr_row = next_row;
                curr_col = next_col;
            }

            curr_node = node.next.as_ref();
        }

        result
    }
}
