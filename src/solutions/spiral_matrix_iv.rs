use crate::solutions::base_solution::Solution;
use crate::utils::linked_list::ListNode;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let n_row = m as usize;
        let n_col = n as usize;
        let mut result = vec![vec![-1; n_col]; n_row];

        let mut curr_row: i32 = 0;
        let mut curr_col: i32 = 0;
        let mut dir_row: i32 = 0;
        let mut dir_col: i32 = 1;

        let mut curr_node = head.as_ref();
        while let Some(node) = curr_node {
            result[curr_row as usize][curr_col as usize] = node.val;

            let (next_row, next_col) = (curr_row + dir_row, curr_col + dir_col);
            if next_row < 0 || next_row as usize >= n_row ||
               next_col < 0 || next_col as usize >= n_col ||
               result[next_row as usize][next_col as usize] != -1 {
                let tmp = dir_row;
                dir_row = dir_col;
                dir_col = -tmp;
            }
            curr_row += dir_row;
            curr_col += dir_col;

            curr_node = node.next.as_ref();
        }

        result
    }
}
