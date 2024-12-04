/**
 * [3371. Identify the Largest Outlier in an Array](https://leetcode.com/problems/identify-the-largest-outlier-in-an-array/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        Solution::get_largest_outlier_v2(nums)
    }

    pub fn get_largest_outlier_v1(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut sum = 0;
        for &v in &nums {
            *counts.entry(v).or_insert(0) += 1;
            sum += v;
        }
        let mut ret = i32::MIN;
        for (&k, &c) in &counts {
            let v = sum - k;
            if v % 2 != 0 {
                continue;
            }
            let half = v / 2;
            if ((half == k && c > 1) || (half != k && counts.contains_key(&half))) && ret < k {
                ret = k;
            }
        }
        ret
    }

    /**
     * 用定长数组代替Map（仅在题目限定的条件下有效）
     */
    pub fn get_largest_outlier_v2(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 2001];
        let mut sum = 0;
        for &v in &nums {
            counts[(v + 1000) as usize] += 1;
            sum += v;
        }
        let mut ret = i32::MIN;
        for (idx, &c) in counts.iter().enumerate() {
            if c == 0 {
                continue;
            }
            let real_value = idx as i32 - 1000;
            let v = sum - real_value;
            if v % 2 != 0 || !(-2000..=2000).contains(&v) {
                continue;
            }
            let half_index = (v / 2 + 1000) as usize;
            if ((half_index == idx && c > 1) || (half_index != idx && counts[half_index] >= 1))
                && ret < real_value
            {
                ret = real_value;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![2, 3, 5, 10];
        assert_eq!(Solution::get_largest_outlier(nums), 10);
    }

    #[test]
    fn test_case2() {
        let nums = vec![-2, -1, -3, -6, 4];
        assert_eq!(Solution::get_largest_outlier(nums), 4);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 1, 1, 1, 1, 5, 5];
        assert_eq!(Solution::get_largest_outlier(nums), 5);
    }

    #[test]
    fn test_case4() {
        let nums = vec![6, -31, 50, -35, 41, 37, -42, 13];
        assert_eq!(Solution::get_largest_outlier(nums), -35);
    }

    #[test]
    fn test_case5() {
        let nums = vec![-947, -326, 200, -747];
        assert_eq!(Solution::get_largest_outlier(nums), -326);
    }
}
