/**
 * [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/description/)
 */
pub struct Solution;

use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => {
                    right = mid.saturating_sub(1);
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }
        if target > nums[left] {
            (left + 1) as i32
        } else {
            left as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 5), 2);
    }

    #[test]
    fn test_case2() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 2), 1);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 7), 4);
    }

    #[test]
    fn test_case4() {
        let nums = vec![];
        assert_eq!(Solution::search_insert(nums, 1), 0);
    }
}
