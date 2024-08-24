use leetcode_practice::solutions::two_keys_keyboard;

#[test]
fn test_two_keys_keyboard_public_1() {
    assert_eq!(two_keys_keyboard::Solution::min_steps(1), 0);
}

#[test]
fn test_two_keys_keyboard_public_2() {
    assert_eq!(two_keys_keyboard::Solution::min_steps(3), 3);
}
