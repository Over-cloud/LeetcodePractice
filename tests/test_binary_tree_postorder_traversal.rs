use leetcode_practice::solutions::base_solution::Solution;
use leetcode_practice::utils::btree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_binary_tree_postorder_traversal_public_1() {
    // Create nodes
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_left_child = Rc::new(RefCell::new(TreeNode::new(3)));

    // Connect nodes
    root.borrow_mut().right = Some(right_child.clone());
    right_child.borrow_mut().left = Some(right_left_child.clone());

    // The tree now looks like this:
    //       1
    //        \
    //         2
    //        /
    //       3

    println!("{:?}", root);
    let expected = vec![3, 2, 1];
    assert_eq!(Solution::postorder_traversal(Some(root)), expected);
}

#[test]
fn test_binary_tree_postorder_traversal_public_2() {
    let expected = Vec::new();
    assert_eq!(Solution::postorder_traversal(None), expected);
}

#[test]
fn test_binary_tree_postorder_traversal_public_3() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let expected = vec![1];
    assert_eq!(Solution::postorder_traversal(Some(root)), expected);
}
