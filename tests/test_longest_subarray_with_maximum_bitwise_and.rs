use leetcode_practice::solutions::base_solution::Solution;


#[test]
fn test_longest_subarray_with_maximum_bitwise_and_public_1() {
    let nums = vec![1,2,3,3,2,2];
    let expected = 2;
    assert_eq!(Solution::longest_subarray(nums), expected);
}

#[test]
fn test_longest_subarray_with_maximum_bitwise_and_public_2() {
    let nums = vec![1,2,3,4];
    let expected = 1;
    assert_eq!(Solution::longest_subarray(nums), expected);
}
