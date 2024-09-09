use leetcode_practice::solutions::base_solution::Solution;
use leetcode_practice::utils::linked_list::ListNode;

#[test]
fn test_spiral_matrix_iv_public_1() {
    let m = 3;
    let n = 5;
    let list_vec = vec![3,0,2,6,8,1,7,9,4,2,5,5,0];
    let head = ListNode::from_vec(list_vec);
    let expected = vec![
        vec![3,0,2,6,8],
        vec![5,0,-1,-1,1],
        vec![5,2,4,9,7]
    ];
    assert_eq!(Solution::spiral_matrix(m, n, head), expected);
}

#[test]
fn test_spiral_matrix_iv_public_2() {
    let m = 1;
    let n = 4;
    let list_vec = vec![0,1,2];
    let head = ListNode::from_vec(list_vec);
    let expected = vec![
        vec![0,1,2,-1],
    ];
    assert_eq!(Solution::spiral_matrix(m, n, head), expected);
}