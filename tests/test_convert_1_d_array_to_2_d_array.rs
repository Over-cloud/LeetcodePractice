use leetcode_practice::solutions::base_solution::Solution;


#[test]
fn test_convert_1_d_array_to_2_d_array_public_1() {
    let original = vec![1,2,3,4];
    let m = 2;
    let n = 2;
    let expected = vec![
        vec![1,2],
        vec![3,4]
    ];
    assert_eq!(Solution::construct2_d_array(original, m, n), expected);
}

#[test]
fn test_convert_1_d_array_to_2_d_array_public_2() {
    let original = vec![1,2,3];
    let m = 1;
    let n = 3;
    let expected = vec![
        vec![1,2,3],
    ];
    assert_eq!(Solution::construct2_d_array(original, m, n), expected);
}

#[test]
fn test_convert_1_d_array_to_2_d_array_public_3() {
    let original = vec![1,2];
    let m = 1;
    let n = 1;
    let expected : Vec<Vec<i32>> = Vec::new();
    assert_eq!(Solution::construct2_d_array(original, m, n), expected);
}
