use leetcode_practice::solutions::base_solution::Solution;


#[test]
fn test_kth_smallest_in_lexicographical_order_public_1() {
    let n = 13;
    let k = 2;
    let expected = 10;
    assert_eq!(Solution::find_kth_number(n, k), expected);
}

#[test]
fn test_kth_smallest_in_lexicographical_order_public_2() {
    let n = 1;
    let k = 1;
    let expected = 1;
    assert_eq!(Solution::find_kth_number(n, k), expected);
}

#[test]
fn test_kth_smallest_in_lexicographical_order_secret_1() {
    let n = 804289384;
    let k = 42641503;
    let expected = 138377349;
    assert_eq!(Solution::find_kth_number(n, k), expected);
}

#[test]
fn test_kth_smallest_in_lexicographical_order_secret_2() {
    let n = 681692778;
    let k = 351251360;
    let expected = 416126219;
    assert_eq!(Solution::find_kth_number(n, k), expected);
}
