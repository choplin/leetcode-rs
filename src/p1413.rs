#[allow(dead_code)]
pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut min: i32 = std::i32::MAX;
    let mut sum: i32 = 0;
    for v in nums.iter() {
        sum = sum + v;
        min = std::cmp::min(min, sum);
    }
    std::cmp::max(-min + 1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1, -2, -3];
        let result = min_start_value(input);
        assert_eq!(result, 5);
    }
}
