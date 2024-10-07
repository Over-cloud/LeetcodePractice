use crate::solutions::base_solution::Solution;
use crate::utils::linked_list::ListNode;

impl Solution {
    pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut curr = head.as_ref();
        let mut len = 0;
        while let Some(boxed_curr) = curr {
            len += 1;
            curr = boxed_curr.next.as_ref();
        }

        let base_len = len / k;
        let addi_len = len % k;
        let mut curr_idx = 0;
        let mut curr_len = 0;
        let mut result = Vec::new();
        while let Some(boxed_curr) = curr {
            if curr_len == 0 {
                result.push(head.take());
            }
            curr_len += 1;

            if curr_idx < addi_len && curr_len == base_len + 1 {
                curr_len = 0;
                curr_idx += 1;
                let curr_tmp = boxed_curr.next.as_mut();
                boxed_curr.next = None;
                curr = curr_tmp;
            } else if curr_idx >= addi_len && curr_len == base_len {
                curr_len = 0;
                curr_idx += 1;
                let curr_tmp = boxed_curr.next.as_mut();
                boxed_curr.next = None;
                curr = curr_tmp;
            } else {
                curr = boxed_curr.next.as_mut();
            }
        }

        return result;
    }
}
