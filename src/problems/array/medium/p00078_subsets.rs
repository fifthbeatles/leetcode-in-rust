/**
 * [78. Subsets](https://leetcode.com/problems/subsets/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length = nums.len();
        let size = 1 << length;
        let mut ret = Vec::with_capacity(size as usize);
        let bits: Vec<_> = (0..length).map(|v| 1 << v).collect();
        for order in 0..size {
            let mut subset = vec![];
            for (idx, &v) in bits.iter().enumerate() {
                if order & v > 0 {
                    subset.push(nums[idx]);
                }
            }
            ret.push(subset);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 3];
        let ret = Solution::subsets(nums);
        assert!(
            ret == vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_case2() {
        let nums = vec![0];
        let ret = Solution::subsets(nums);
        assert!(ret == vec![vec![], vec![0]]);
    }
}
