use leetcode_practice::solutions::base_solution::Solution;
use leetcode_practice::utils::narytree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_nary_tree_postorder_traversal_public_1() {
    // Constructing a sample tree:
    //        1
    //      / | \
    //     3  2  4
    //       / \
    //      5   6

    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let node2 = Rc::new(RefCell::new(TreeNode {
        val: 2,
        children: vec![node5, node6],
    }));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        children: vec![node3, node2, node4],
    }));

    println!("{:?}", root);
    let expected = vec![3, 5, 6, 2, 4, 1];
    assert_eq!(Solution::postorder(root), expected);
}
