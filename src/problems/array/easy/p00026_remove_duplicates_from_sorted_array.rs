/**
 * [26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        let mut pre_val = i32::MAX;
        let mut indexes = Vec::new();
        for (i, v) in nums.iter().enumerate() {
            if pre_val != *v {
                indexes.push(i);
                pre_val = *v;
            }
        }
        for (i1, i2) in indexes.iter().enumerate() {
            nums[i1] = nums[*i2];
        }
        indexes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], vec![1, 2]);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
    }
}
