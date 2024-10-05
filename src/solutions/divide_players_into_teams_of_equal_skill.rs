use crate::solutions::base_solution::Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let len = skill.len();
        let team_cnt = (len / 2) as i64;
        let sum: i64 = skill.iter().map(|val| *val as i64).sum();

        if sum % team_cnt != 0 {
            return -1;
        }
        let team_sum = sum / team_cnt;

        let mut cnt_sort = vec![0; team_sum as usize];
        for val in skill {
            let val = val as usize;
            if val >= cnt_sort.len() {
                return -1;
            }
            cnt_sort[val] += 1;
        }

        let mut result = 0;
        for idx in 1..((cnt_sort.len() + 1) / 2) {
            if cnt_sort[idx] == 0 {
                continue;
            }

            let paired_idx = cnt_sort.len() - idx;
            if cnt_sort[idx] != cnt_sort[paired_idx] {
                return -1;
            }

            result += (idx as i64) * (paired_idx as i64) * cnt_sort[idx];
        }

        if cnt_sort.len() % 2 == 0 {
            let idx = cnt_sort.len() / 2;
            result += (idx as i64) * (idx as i64) * cnt_sort[idx] / 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_players_into_teams_of_equal_skills_public_1() {
        let skill = vec![3,2,5,1,3,4];
        let actual = Solution::divide_players(skill);
        let expected = 22;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_divide_players_into_teams_of_equal_skills_public_2() {
        let skill = vec![3,4];
        let actual = Solution::divide_players(skill);
        let expected = 12;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_divide_players_into_teams_of_equal_skills_public_3() {
        let skill = vec![1,1,2,3];
        let actual = Solution::divide_players(skill);
        let expected = -1;
        assert_eq!(actual, expected);
    }
}
