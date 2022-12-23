struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        let mut maxes = vec![0; prices.len()];

        maxes[0] = 0;
        maxes[1] = std::cmp::max(prices[1] - prices[0], 0);

        for i in 2..prices.len() {
            let max_for_sell = (0..i)
                .map(|j| {
                    if j < 2 {
                        prices[i] - prices[j]
                    } else {
                        maxes[j - 2] + prices[i] - prices[j]
                    }
                })
                .max()
                .unwrap();
            maxes[i] = *vec![
                maxes[i - 2], // buy
                maxes[i - 1], // cooldown
                max_for_sell, // sell
            ]
            .iter()
            .max()
            .unwrap();
        }

        maxes[prices.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]))
    }

    #[test]
    fn it_works2() {
        assert_eq!(0, Solution::max_profit(vec![1]))
    }
}
