use leetcode_practice::solutions::base_solution::Solution;

#[test]
fn test_lexicographical_numbers_public_1() {
    let n = 13;
    let expected = vec![1,10,11,12,13,2,3,4,5,6,7,8,9];
    assert_eq!(Solution::lexical_order(n), expected);
}

#[test]
fn test_lexicographical_numbers_public_2() {
    let n = 2;
    let expected = vec![1,2];
    assert_eq!(Solution::lexical_order(n), expected);
}
