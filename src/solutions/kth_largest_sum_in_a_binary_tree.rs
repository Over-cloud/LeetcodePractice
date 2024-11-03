use crate::solutions::base_solution::Solution;
use crate::utils::btree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{BinaryHeap, LinkedList};
use std::cmp::Reverse;


impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut min_heap = BinaryHeap::new();
        let mut queue = LinkedList::new();

        if let Some(root_node) = root {
            queue.push_back(root_node);
        }

        while queue.len() > 0 {
            let mut level_sum : i64 = 0;
            for _ in 0..queue.len() {
                let current_node = queue.pop_front().unwrap();
                let current = current_node.borrow();
                level_sum += current.val as i64;
                if let Some(left_node) = current.left.as_ref() {
                    queue.push_back(Rc::clone(left_node));
                }
                if let Some(right_node) = current.right.as_ref() {
                    queue.push_back(Rc::clone(right_node));
                }
            }
            min_heap.push(Reverse(level_sum));
            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }

        if min_heap.len() < k as usize {
            -1
        } else {
            min_heap.peek().unwrap().0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest_level_sum_public_1() {
        // Create nodes
        let root = Rc::new(RefCell::new(TreeNode::new(5)));
        let left_child = Rc::new(RefCell::new(TreeNode::new(8)));
        let left_left_child = Rc::new(RefCell::new(TreeNode::new(2)));
        let left_left_left_child = Rc::new(RefCell::new(TreeNode::new(4)));
        let left_left_right_child = Rc::new(RefCell::new(TreeNode::new(6)));
        let left_right_child = Rc::new(RefCell::new(TreeNode::new(1)));
        let right_child = Rc::new(RefCell::new(TreeNode::new(9)));
        let right_left_child = Rc::new(RefCell::new(TreeNode::new(3)));
        let right_right_child = Rc::new(RefCell::new(TreeNode::new(7)));


    // Connect nodes
    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());

    left_child.borrow_mut().left = Some(left_left_child.clone());
    left_child.borrow_mut().right = Some(left_right_child.clone());

    left_left_child.borrow_mut().left = Some(left_left_left_child.clone());
    left_left_child.borrow_mut().right = Some(left_left_right_child.clone());

    right_child.borrow_mut().left = Some(right_left_child.clone());
    right_child.borrow_mut().right = Some(right_right_child.clone());


    // The tree now looks like this:
    //           5
    //         /   \
    //        8     9
    //       / \   / \
    //      2   1 3   7
    //     / \
    //    4   6

    println!("{:?}", root);
    let expected = 13;
    assert_eq!(Solution::kth_largest_level_sum(Some(root), 2), expected);
    }
}
