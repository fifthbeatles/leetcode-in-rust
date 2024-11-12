/**
 * [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/description/)
 */
pub struct Solution;

use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        Self::contains_duplicate_v3(nums)
    }

    pub fn contains_duplicate_v1(nums: Vec<i32>) -> bool {
        let nums_set: HashSet<_> = nums.iter().collect();
        nums_set.len() != nums.len()
    }

    pub fn contains_duplicate_v2(nums: Vec<i32>) -> bool {
        let mut nums_set = HashSet::new();
        !nums.iter().all(|v| nums_set.insert(v))
    }

    pub fn contains_duplicate_v3(nums: Vec<i32>) -> bool {
        let mut nums_set = HashSet::with_capacity(nums.len());
        !nums.iter().all(|v| nums_set.insert(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::contains_duplicate(nums), true);
    }

    #[test]
    fn test_case2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::contains_duplicate(nums), false);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(Solution::contains_duplicate(nums), true);
    }
}
