use leetcode_practice::solutions::base_solution::Solution;

#[test]
fn test_count_the_number_of_consistent_strings_public_1() {
    let allowed = "ab".to_string();
    let words = vec!["ad","bd","aaab","baa","badab"].into_iter().map(String::from).collect();
    let expected = 2;
    assert_eq!(Solution::count_consistent_strings(allowed, words), expected);
}

#[test]
fn test_count_the_number_of_consistent_strings_public_2() {
    let allowed = "abc".to_string();
    let words = vec!["a","b","c","ab","ac","bc","abc"].into_iter().map(String::from).collect();
    let expected = 7;
    assert_eq!(Solution::count_consistent_strings(allowed, words), expected);
}

#[test]
fn test_count_the_number_of_consistent_strings_public_3() {
    let allowed = "cad".to_string();
    let words = vec!["cc","acd","b","ba","bac","bad","ac","d"].into_iter().map(String::from).collect();
    let expected = 4;
    assert_eq!(Solution::count_consistent_strings(allowed, words), expected);
}
