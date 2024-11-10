/**
 * [27. Remove Element](https://leetcode.com/problems/remove-element/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let mut j = 0;
        for i in 0..nums.len() {
            if val != nums[i] {
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        nums[..2].sort();
        assert_eq!(nums[..2], vec![2, 2]);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        nums[..5].sort();
        assert_eq!(nums[..5], vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn test_case3() {
        let mut nums = vec![];
        assert_eq!(Solution::remove_element(&mut nums, 0), 0);
    }

    #[test]
    fn test_case4() {
        let mut nums = vec![1];
        assert_eq!(Solution::remove_element(&mut nums, 1), 0);
    }

    #[test]
    fn test_case5() {
        let mut nums = vec![1];
        assert_eq!(Solution::remove_element(&mut nums, 0), 1);
    }
}
