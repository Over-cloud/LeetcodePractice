use leetcode_practice::solutions::base_solution::Solution;


#[test]
fn test_most_stones_removed_with_same_row_or_column_public_1() {
    let stones = vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 2],
        vec![2, 1],
        vec![2, 2],
    ];
    let expected = 5;
    assert_eq!(Solution::remove_stones(stones), expected);
}

#[test]
fn test_most_stones_removed_with_same_row_or_column_public_2() {
    let stones = vec![
        vec![0, 0],
        vec![0, 2],
        vec![1, 1],
        vec![2, 0],
        vec![2, 2],
    ];
    let expected = 3;
    assert_eq!(Solution::remove_stones(stones), expected);
}

#[test]
fn test_most_stones_removed_with_same_row_or_column_public_3() {
    let stones = vec![
        vec![0, 0],
    ];
    let expected = 0;
    assert_eq!(Solution::remove_stones(stones), expected);
}
