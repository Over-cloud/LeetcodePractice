use leetcode_practice::solutions::base_solution::Solution;


#[test]
fn test_find_the_student_that_will_replace_the_chalk_public_1() {
    let chalk = vec![5,1,5];
    let k = 22;
    let expected = 0;
    assert_eq!(Solution::chalk_replacer(chalk,k), expected);
}

#[test]
fn test_find_the_student_that_will_replace_the_chalk_public_2() {
    let chalk = vec![3,4,1,2];
    let k = 25;
    let expected = 1;
    assert_eq!(Solution::chalk_replacer(chalk,k), expected);
}
