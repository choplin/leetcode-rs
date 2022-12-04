struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let sum: i64 = nums.iter().map(|&n| n as i64).sum();
        let mut left = 0i64;
        let mut right = sum;

        let mut min = i64::MAX;
        let mut min_index: usize = 0;

        for (i, &v) in nums.iter().enumerate() {
            left += v as i64;
            right -= v as i64;
            let average_difference = if i == len - 1 {
                left / ((i + 1) as i64)
            } else {
                i64::abs((left / ((i + 1) as i64)) - (right / ((len - i - 1) as i64)))
            };

            if average_difference < min {
                min = average_difference;
                min_index = i;
            }
        }
        min_index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3])
        )
    }
}
