use crate::solutions::base_solution::Solution;
use crate::utils::btree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::LinkedList;


impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let level_sums = Self::replace_value_in_tree_aux_get_level_sum(root.clone());

        let mut queue = LinkedList::new();
        if let Some(root_rc) = root.clone() {
            root_rc.borrow_mut().val = 0;
            queue.push_back(root_rc);
        }

        let mut next_level: usize = 1;
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let curr_rc = queue.pop_front().unwrap();
                let mut curr = curr_rc.borrow_mut();

                let mut sum = 0;
                if let Some(left) = curr.left.as_ref() {
                    sum += left.borrow().val;
                    queue.push_back(left.clone());
                }
                if let Some(right) = curr.right.as_ref() {
                    sum += right.borrow().val;
                    queue.push_back(right.clone());
                }

                if let Some(left) = curr.left.as_mut() {
                    let mut left = left.borrow_mut();
                    left.val = level_sums[next_level] - sum;
                }
                if let Some(right) = curr.right.as_mut() {
                    let mut right = right.borrow_mut();
                    right.val = level_sums[next_level] - sum;
                }
            }

            next_level += 1;
        }

        root
    }

    fn replace_value_in_tree_aux_get_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut level_sums = Vec::new();
        let mut queue = LinkedList::new();
        if let Some(node) = root.clone() {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let len = queue.len();
            let mut level_sum = 0;
            for _ in 0..len {
                let curr_rc = queue.pop_front().unwrap();
                let curr = curr_rc.borrow();
                level_sum += curr.val;
                if let Some(left) = curr.left.as_ref() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = curr.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }

            level_sums.push(level_sum);
        }

        level_sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_value_in_tree_public_1() {
        // Create nodes
        let root = Rc::new(RefCell::new(TreeNode::new(5)));
        let left_child = Rc::new(RefCell::new(TreeNode::new(4)));
        let right_child = Rc::new(RefCell::new(TreeNode::new(9)));
        let left_left_child = Rc::new(RefCell::new(TreeNode::new(1)));
        let left_right_child = Rc::new(RefCell::new(TreeNode::new(10)));
        let right_right_child = Rc::new(RefCell::new(TreeNode::new(7)));

        // Connect nodes
        root.borrow_mut().left = Some(left_child.clone());
        root.borrow_mut().right = Some(right_child.clone());
        left_child.borrow_mut().left = Some(left_left_child.clone());
        left_child.borrow_mut().right = Some(left_right_child.clone());
        right_child.borrow_mut().right = Some(right_right_child.clone());

        // Create nodes
        let expected_root = Rc::new(RefCell::new(TreeNode::new(0)));
        let left_child = Rc::new(RefCell::new(TreeNode::new(0)));
        let right_child = Rc::new(RefCell::new(TreeNode::new(0)));
        let left_left_child = Rc::new(RefCell::new(TreeNode::new(7)));
        let left_right_child = Rc::new(RefCell::new(TreeNode::new(7)));
        let right_right_child = Rc::new(RefCell::new(TreeNode::new(11)));

        // Connect nodes
        expected_root.borrow_mut().left = Some(left_child.clone());
        expected_root.borrow_mut().right = Some(right_child.clone());
        left_child.borrow_mut().left = Some(left_left_child.clone());
        left_child.borrow_mut().right = Some(left_right_child.clone());
        right_child.borrow_mut().right = Some(right_right_child.clone());

        assert_eq!(Solution::replace_value_in_tree(Some(root)), Some(expected_root));
    }

    #[test]
    fn test_replace_value_in_tree_public_2() {
        // Create nodes
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        let left_child = Rc::new(RefCell::new(TreeNode::new(1)));
        let right_child = Rc::new(RefCell::new(TreeNode::new(2)));

        // Connect nodes
        root.borrow_mut().left = Some(left_child.clone());
        root.borrow_mut().right = Some(right_child.clone());

        // Create nodes
        let expected_root = Rc::new(RefCell::new(TreeNode::new(0)));
        let left_child = Rc::new(RefCell::new(TreeNode::new(0)));
        let right_child = Rc::new(RefCell::new(TreeNode::new(0)));

        // Connect nodes
        expected_root.borrow_mut().left = Some(left_child.clone());
        expected_root.borrow_mut().right = Some(right_child.clone());

        assert_eq!(Solution::replace_value_in_tree(Some(root)), Some(expected_root));
    }
}