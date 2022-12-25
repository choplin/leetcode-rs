struct Solution {}

impl Solution {
    #[allow(dead_code)]

    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort();

        let mut queries: Vec<(&i32, usize)> = queries.iter().zip(0..queries.len()).collect();
        queries.sort_by(|a, b| a.0.cmp(b.0));

        let mut answer = Vec::with_capacity(queries.len());
        let mut count = 0;
        let mut sum = 0;

        'outer: for q in queries {
            while sum <= *q.0 {
                if count == nums.len() {
                    answer.push((count, q.1));
                    continue 'outer;
                }
                sum += nums[count];
                count += 1;
            }
            answer.push((count - 1, q.1));
        }

        answer.sort_by(|a, b| a.1.cmp(&b.1));
        answer.iter().map(|e| e.0 as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 3, 4],
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21])
        )
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            vec![2, 2, 1, 1, 2, 3, 3],
            Solution::answer_queries(
                vec![736411, 184882, 914641, 37925, 214915],
                vec![331244, 273144, 118983, 118252, 305688, 718089, 665450]
            )
        )
    }
}
