use crate::solutions::base_solution::Solution;
use crate::utils::narytree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder(root: Rc<RefCell<TreeNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::postorder_aux(&root, &mut result);
        result
    }

    fn postorder_aux(root: &Rc<RefCell<TreeNode>>, array: &mut Vec<i32>) {
        let root_ref = root.borrow();
        for child in &root_ref.children {
            Self::postorder_aux(child, array);
        }
        array.push(root_ref.val);
    }
}