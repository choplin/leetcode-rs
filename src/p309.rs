struct Solution {}

use std::cmp::max;

impl Solution {
    #[allow(unused)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let cooldown = 1;
        let mut profits = vec![0; prices.len()];
        let mut max_profit = 0;
        let mut tmp_max = profits[0] - prices[0];

        for i in 1..prices.len() {
            profits[i] = max(profits[i - 1], tmp_max + prices[i]);
            tmp_max = max(
                tmp_max,
                if i > cooldown {
                    profits[i - 1 - cooldown]
                } else {
                    profits[i - 1]
                } - prices[i],
            );
            max_profit = max(profits[i], max_profit);
        }
        max_profit
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
