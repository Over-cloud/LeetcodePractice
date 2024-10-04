use crate::solutions::base_solution::Solution;

enum BribeResult {
    Chaos(&'static str),
    Count(i32),
}

impl Solution {
    fn minimum_bribes(q: &[i32]) -> BribeResult {
        let mut bribes = 0;
        for i in 0..q.len() {
            let pos = i as i32 + 1;
            let val = q[i];
            
            if val - pos > 2 {
                return BribeResult::Chaos("Too chaotic");
            }
            
            let start = 0.max(val - 2) as usize;
            for j in start..i {
                if q[j] > val {
                    bribes += 1;
                }
            }
        }

        return BribeResult::Count(bribes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_bribes_public_1() {
        let q = vec![2, 1, 5, 3, 4];
        let expected = 3;
        match Solution::minimum_bribes(&q) {
            BribeResult::Count(actual) => assert_eq!(actual, expected),
            _ => panic!("Expected a Number variant"),
        }
    }

    #[test]
    fn test_minimum_bribes_public_2() {
        let q = vec![2, 5, 1, 3, 4];
        let expected = "Too chaotic";
        match Solution::minimum_bribes(&q) {
            BribeResult::Chaos(actual) => assert_eq!(actual, expected),
            _ => panic!("Expected a String variant"),
        }
    }

    #[test]
    fn test_minimum_bribes_public_3() {
        let q = vec![5, 1, 2, 3, 7, 8, 6, 4];
        let expected = "Too chaotic";
        match Solution::minimum_bribes(&q) {
            BribeResult::Chaos(actual) => assert_eq!(actual, expected),
            _ => panic!("Expected a String variant"),
        }
    }

    #[test]
    fn test_minimum_bribes_public_4() {
        let q = vec![1, 2, 5, 3, 7, 8, 6, 4];
        let expected = 7;
        match Solution::minimum_bribes(&q) {
            BribeResult::Count(actual) => assert_eq!(actual, expected),
            _ => panic!("Expected a Number variant"),
        }
    }

    #[test]
    fn test_minimum_bribes_public_5() {
        let q = vec![1, 2, 5, 3, 4, 7, 8, 6];
        let expected = 4;
        match Solution::minimum_bribes(&q) {
            BribeResult::Count(actual) => assert_eq!(actual, expected),
            _ => panic!("Expected a Number variant"),
        }
    }
}
