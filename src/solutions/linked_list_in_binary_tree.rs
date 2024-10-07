use crate::solutions::base_solution::Solution;
use crate::utils::btree::TreeNode;
use crate::utils::linked_list::ListNode;

use std::rc::Rc;
use std::cell::RefCell;

// TLE
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_sub_path_dfs(&head, &head, &root)
    }

    pub fn is_sub_path_dfs(list_head: &Option<Box<ListNode>>,
                           list_curr: &Option<Box<ListNode>>,
                           tree_curr: &Option<Rc<RefCell<TreeNode>>>) -> bool {
       
        match list_curr.as_ref() {
            Some(boxed_list_curr) => {
                match tree_curr.as_ref() {
                    Some(rc_tree_curr) => {
                        let tree_curr = rc_tree_curr.borrow();
                        if boxed_list_curr.val == tree_curr.val {
                            if Self::is_sub_path_dfs(list_head, &boxed_list_curr.next, &tree_curr.left) ||
                               Self::is_sub_path_dfs(list_head, &boxed_list_curr.next, &tree_curr.right) {
                                return true;
                            }
                            return Self::is_sub_path_dfs(list_head, list_head, &tree_curr.left) ||
                                    Self::is_sub_path_dfs(list_head, list_head, &tree_curr.right);

                        } else {
                            if let Some(boxed_list_head) = list_head.as_ref() {
                                if boxed_list_head.val == tree_curr.val {
                                    return Self::is_sub_path_dfs(list_head, &boxed_list_head.next, &tree_curr.left) ||
                                           Self::is_sub_path_dfs(list_head, &boxed_list_head.next, &tree_curr.right);
                                }
                            }
                            return Self::is_sub_path_dfs(list_head, list_head, &tree_curr.left) ||
                                   Self::is_sub_path_dfs(list_head, list_head, &tree_curr.right);
                        }
                    },

                    None => false,
                }
            },

            None => true,
        }
    }
}
