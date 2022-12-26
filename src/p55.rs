struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let last_index = nums.len() - 1;
        let mut max_reachable = 0;
        for (i, s) in nums.iter().enumerate() {
            max_reachable = max_reachable.max(i + *s as usize);
            if max_reachable >= last_index {
                return true;
            } else if max_reachable == i {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]))
    }

    #[test]
    fn it_works2() {
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]))
    }

    #[test]
    fn it_works3() {
        assert_eq!(true, Solution::can_jump(vec![0]))
    }
}
