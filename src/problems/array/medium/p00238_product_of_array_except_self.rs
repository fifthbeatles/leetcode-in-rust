/**
 * [238. Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut products = vec![0; size];
        products[0] = 1;
        let mut prefix = 1;
        for idx in 0..size - 1 {
            prefix *= nums[idx];
            products[idx + 1] = prefix;
        }
        let mut suffix = 1;
        for idx in (0..size).rev() {
            products[idx] *= suffix;
            suffix *= nums[idx];
        }
        products
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::product_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_case2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(Solution::product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn test_case3() {
        let nums = vec![-1, 1, 0, 0, 3];
        assert_eq!(Solution::product_except_self(nums), vec![0, 0, 0, 0, 0]);
    }
}
