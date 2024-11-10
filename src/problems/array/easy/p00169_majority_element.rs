/**
 * [169. Majority Element](https://leetcode.com/problems/majority-element/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        Self::majority_element_v1(nums)
    }

    pub fn majority_element_v1(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let half = (nums.len() / 2) as i32;
        for k in nums {
            let count = map.entry(k).or_insert(0);
            *count += 1;
            if *count > half {
                return k;
            }
        }
        0
    }

    pub fn majority_element_v2(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut majority = 0;
        for x in nums {
            if count == 0 {
                majority = x;
                count = 1;
            } else if x == majority {
                count += 1;
            } else {
                count -= 1;
            }
        }
        majority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
