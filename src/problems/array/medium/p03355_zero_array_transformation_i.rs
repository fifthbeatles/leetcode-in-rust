/**
 * [3355. Zero Array Transformation I](https://leetcode.com/problems/zero-array-transformation-i/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        Solution::is_zero_array_v2(nums, queries)
    }

    pub fn is_zero_array_v1(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let size = nums.len();
        // 创建差分数组
        let mut diff = vec![0; size + 1];
        // 相当于在nums前面插入了一个0
        diff[0] = nums[0];
        for idx in 1..size {
            diff[idx] = nums[idx] - nums[idx - 1];
        }
        // 相当于在nums后面插入了一个0
        diff[size] = 0 - nums[size - 1];
        for query in &queries {
            diff[query[0] as usize] -= 1;
            diff[query[1] as usize + 1] += 1;
        }
        let mut sum = 0;
        for d in &diff {
            sum += d;
            if sum > 0 {
                return false;
            }
        }
        true
    }

    pub fn is_zero_array_v2(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let size = nums.len();
        let mut diff = vec![0; size + 1];

        for query in queries {
            diff[query[0] as usize] += 1;
            diff[query[1] as usize + 1] -= 1;
        }

        let mut sum = 0;
        for i in 0..size {
            sum += diff[i];
            if nums[i] > sum {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 0, 1];
        let queries = vec![vec![0, 2]];
        assert_eq!(Solution::is_zero_array(nums, queries), true);
    }

    #[test]
    fn test_case2() {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3], vec![0, 2]];
        assert_eq!(Solution::is_zero_array(nums, queries), false);
    }
}
