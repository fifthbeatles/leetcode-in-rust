/**
 * [704. Binary Search](https://leetcode.com/problems/binary-search/description/)
 */
pub struct Solution;

use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_v2(nums, target)
    }

    pub fn search_v1(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        -1
    }

    pub fn search_v2(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => right = mid,
                Ordering::Less => left = mid + 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search(nums, 9), 4);
    }

    #[test]
    fn test_case2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search(nums, 2), -1);
    }
}
