use leetcode_practice::solutions::base_solution::Solution;

#[test]
fn test_sort_the_people_public_1() {
    let names = vec!["Mary","John","Emma"].into_iter().map(String::from).collect::<Vec<String>>();
    let heights = vec![180,165,170];

    let sorted_names = vec!["Mary","Emma","John"].into_iter().map(String::from).collect::<Vec<String>>();
    assert_eq!(Solution::sort_people(names, heights), sorted_names);
}

#[test]
fn test_sort_the_people_public_2() {
    let names = vec!["Alice","Bob","Bob"].into_iter().map(String::from).collect::<Vec<String>>();
    let heights = vec![155,185,150];

    let sorted_names = vec!["Bob","Alice","Bob"].into_iter().map(String::from).collect::<Vec<String>>();
    assert_eq!(Solution::sort_people(names, heights), sorted_names);
}
