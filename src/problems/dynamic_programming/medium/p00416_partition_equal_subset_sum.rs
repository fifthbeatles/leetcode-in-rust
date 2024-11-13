/**
 * [416. Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/description/)
 */
pub struct Solution;

use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        Solution::can_partition_v2(nums)
    }

    pub fn can_partition_v1(nums: Vec<i32>) -> bool {
        let nums: Vec<usize> = nums.iter().map(|v| *v as usize).collect();
        let s: usize = nums.iter().sum();
        if s % 2 != 0 {
            return false;
        }
        let target = s / 2;
        if *(nums.iter().max().unwrap()) > target {
            return false;
        }
        let size = nums.len();
        // dp[i][j]表示nums[0..i]能否合成j
        let mut dp = vec![vec![false; target + 1]; size + 1];
        for i in 1..=size {
            for j in 1..=target {
                dp[i][j] = match nums[i - 1].cmp(&j) {
                    Ordering::Equal => true,
                    Ordering::Greater => dp[i - 1][j],
                    Ordering::Less => dp[i - 1][j] || dp[i - 1][j - nums[i - 1]],
                };
            }
        }
        dp[size][target]
    }

    pub fn can_partition_v2(nums: Vec<i32>) -> bool {
        let nums: Vec<usize> = nums.iter().map(|v| *v as usize).collect();
        let s: usize = nums.iter().sum();
        if s % 2 != 0 {
            return false;
        }
        let target = s / 2;
        if *(nums.iter().max().unwrap()) > target {
            return false;
        }
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for &num in nums.iter() {
            for index in (num..=target).rev() {
                dp[index] = dp[index] || dp[index - num];
            }
        }
        dp[target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 5, 11, 5];
        assert_eq!(Solution::can_partition(nums), true);
    }

    #[test]
    fn test_case2() {
        let nums = vec![1, 2, 3, 5];
        assert_eq!(Solution::can_partition(nums), false);
    }

    #[test]
    fn test_case3() {
        let nums = vec![2, 2, 3, 5];
        assert_eq!(Solution::can_partition(nums), false);
    }
}
