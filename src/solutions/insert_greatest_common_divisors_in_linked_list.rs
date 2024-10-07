use crate::solutions::base_solution::Solution;
use crate::utils::linked_list::ListNode;

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let curr = head.as_mut();

        while let Some(curr_node) = curr {
            let mut next_node = curr_node.next.take();

            let dummy = Box::new(ListNode {
                next: next_node,
                val: 0,
            });

            curr_node.next = Some(dummy);

            curr = curr_node.next.as_mut().unwrap().next.as_mut();
        }

        head
    }
}
