/**
 * [136. Single Number](https://leetcode.com/problems/single-number/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().reduce(|acc, cur| acc ^ cur).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
