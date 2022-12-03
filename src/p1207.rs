struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count = [0; 2001];
        for v in arr.iter() {
            let index = (*v + 1000) as usize;
            count[index] += 1;
        }
        let mut seen = [false; 1000];
        for c in count.iter() {
            if *c == 0 {
                continue;
            } else {
                let index = *c as usize;
                if seen[index] {
                    return false;
                } else {
                    seen[index] = true;
                }
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
        assert_eq!(true, Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert_eq!(false, Solution::unique_occurrences(vec![1, 2]));
        assert_eq!(
            true,
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0])
        );
    }
}
