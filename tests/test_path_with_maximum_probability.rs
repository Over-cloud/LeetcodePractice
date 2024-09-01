use leetcode_practice::solutions::base_solution::Solution;

#[test]
fn test_path_with_maximum_probability_public_1() {
    let n = 3;
    let edges = vec![
        vec![0, 1],
        vec![1, 2],
        vec![0, 2],
    ];
    let succ_prob = vec![0.5, 0.5, 0.2];
    let start_node = 0;
    let end_node = 2;
    let expected = 0.25;
    assert_eq!(Solution::max_probability(n, edges, succ_prob, start_node, end_node), expected);
}
