use std::cmp::Ordering;

/**
 * [33. Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let length = nums.len();
        let mut left = 0;
        let mut right = length;
        while left < right {
            let mid = (left + right) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => {
                    if nums[mid] > nums[right - 1] {
                        // 分界点在mid右边
                        left = mid + 1;
                    } else {
                        // 分界点在mid右边
                        if target > nums[right - 1] {
                            right = mid;
                        } else {
                            left = mid + 1;
                        }
                    }
                }
                Ordering::Less => {
                    if nums[mid] < nums[left] {
                        // 分界点在mid左边
                        right = mid;
                    } else {
                        // 分界点在mid右边
                        if target < nums[left] {
                            left = mid + 1;
                        } else {
                            right = mid;
                        }
                    }
                }
            };
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::search(nums, 0), 4);
    }

    #[test]
    fn test_case2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::search(nums, 3), -1);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1];
        assert_eq!(Solution::search(nums, 0), -1);
    }

    #[test]
    fn test_case4() {
        let nums = vec![3, 5, 1];
        assert_eq!(Solution::search(nums, 3), 0);
    }

    #[test]
    fn test_case5() {
        let nums = vec![7, 8, 1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::search(nums, 2), 3);
    }
}
