pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn beauty_sum(s: String) -> i32 {
        let len = s.len();
        let mut sum = 0;
        let mut b = [0; 2];
        for start in 0..len - 1 {
            let mut count: [i32; 26] = [0; 26];
            let mut min: i32 = 1;
            let mut max: i32 = 1;
            for c in s.chars().skip(start) {
                c.encode_utf8(&mut b);
                let index = usize::from(b[0] - 97);
                let v = count[index];
                count[index] = v + 1;

                if v + 1 > max {
                    max = v + 1;
                }
                if v <= min {
                    if *count.iter().filter(|c| **c > 0).min().unwrap() > v {
                        min = v + 1;
                    }
                }
                sum += max - min;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::beauty_sum("aabcbaa".to_string());
        assert_eq!(17, result);
    }
}
