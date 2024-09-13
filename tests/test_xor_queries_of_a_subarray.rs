use leetcode_practice::solutions::base_solution::Solution;

#[test]
fn test_xor_queries_of_a_subarray_public_1() {
    let arr = vec![1,3,4,8];
    let queries = vec![vec![0,1],vec![1,2],vec![0,3],vec![3,3]];
    let expected = vec![2,7,14,8];
    assert_eq!(Solution::xor_queries(arr, queries), expected);
}

#[test]
fn test_xor_queries_of_a_subarray_public_2() {
    let arr = vec![4,8,2,10];
    let queries = vec![vec![2,3],vec![1,3],vec![0,0],vec![0,3]];
    let expected = vec![8,0,4,4];
    assert_eq!(Solution::xor_queries(arr, queries), expected);
}
