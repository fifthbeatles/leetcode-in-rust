/**
 * [322. Coin Change](https://leetcode.com/problems/coin-change/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut counts = vec![None; (amount + 1) as usize];
        counts[0] = Some(0);
        for current_amount in 1..=amount {
            counts[current_amount as usize] = coins
                .iter()
                .filter_map(|unit| {
                    if current_amount < *unit {
                        None
                    } else {
                        counts[(current_amount - *unit) as usize].map(|v| v + 1)
                    }
                })
                .min()
        }
        counts[amount as usize].unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let coins = vec![1, 2, 5];
        assert_eq!(Solution::coin_change(coins, 11), 3);
    }

    #[test]
    fn test_case2() {
        let coins = vec![2];
        assert_eq!(Solution::coin_change(coins, 3), -1);
    }

    #[test]
    fn test_case3() {
        let coins = vec![1];
        assert_eq!(Solution::coin_change(coins, 0), 0);
    }
}
