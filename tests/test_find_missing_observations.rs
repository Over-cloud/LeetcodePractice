use leetcode_practice::solutions::base_solution::Solution;

#[test]
fn test_find_missing_observations_public_1() {
    let rolls = vec![3,2,4,3];
    let mean = 4;
    let n = 2;
    let expected = vec![6,6];
    assert_eq!(Solution::missing_rolls(rolls, mean, n), expected);
}

#[test]
fn test_find_missing_observations_public_2() {
    let rolls = vec![1,5,6];
    let mean = 3;
    let n = 4;
    let expected = vec![2,3,2,2];
    assert_eq!(Solution::missing_rolls(rolls, mean, n).iter().sum::<i32>(), expected.iter().sum());
}

#[test]
fn test_find_missing_observations_public_3() {
    let rolls = vec![1,2,3,4];
    let mean = 6;
    let n = 4;
    let expected = Vec::new();
    assert_eq!(Solution::missing_rolls(rolls, mean, n), expected);
}
