use crate::solutions::base_solution::Solution;
use crate::utils::btree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(rc_node) => {
                let node_ref = rc_node.borrow();
                let mut left = Solution::postorder_traversal(node_ref.left.clone());
                let right = Solution::postorder_traversal(node_ref.right.clone());
                left.extend(right);
                left.push(node_ref.val);
                left
            },
            None => Vec::new(),
        }
    }
}