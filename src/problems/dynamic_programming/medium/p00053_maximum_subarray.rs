/**
 * [53. Maximum Subarray](https://leetcode.com/problems/maximum-subarray/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut ret = nums[0];
        let mut max_sum = nums[0];
        for v in nums.iter().skip(1) {
            max_sum = (max_sum + *v).max(*v);
            ret = ret.max(max_sum);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
