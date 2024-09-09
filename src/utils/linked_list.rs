// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    // #[inline]
    // fn new(val: i32) -> Self {
    //     ListNode {
    //         next: None,
    //         val
    //     }
    // }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        for &val in vec.iter().rev() {
            let node = Box::new(ListNode {
                next: head,
                val: val,
            });
            head = Some(node);
        }

        head
    }
}
