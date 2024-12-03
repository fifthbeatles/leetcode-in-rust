/**
 * [3356. Zero Array Transformation II](https://leetcode.com/problems/zero-array-transformation-ii/description/)
 */
pub struct Solution;

use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let size = nums.len();
        let query_size = queries.len();
        let mut diff = vec![0; size + 1];
        // 相当于在nums前面插入了一个0
        diff[0] = nums[0];
        for idx in 1..size {
            diff[idx] = nums[idx] - nums[idx - 1];
        }
        // 相当于在nums后面插入了一个0
        diff[size] = 0 - nums[size - 1];
        let mut current_pos = 0;
        // 最开始时预计算存储了所有差分数组，但是会超时，所以改为每次基于前一次的情况进行调整，如果小了就回溯
        let pos = (0..=query_size)
            .collect::<Vec<_>>()
            .binary_search_by(|&idx| {
                if idx > current_pos {
                    while current_pos < idx {
                        let query = &queries[current_pos];
                        let decrement = query[2];
                        diff[query[0] as usize] -= decrement;
                        diff[query[1] as usize + 1] += decrement;
                        current_pos += 1;
                    }
                } else {
                    while current_pos > idx {
                        current_pos -= 1;
                        let query = &queries[current_pos];
                        let decrement = query[2];
                        diff[query[0] as usize] += decrement;
                        diff[query[1] as usize + 1] -= decrement;
                    }
                }
                let mut sum = 0;
                for &d in &diff {
                    sum += d;
                    if sum > 0 {
                        return Ordering::Less;
                    }
                }
                // 需要找到第一个减到0的位置，所以要继续往前找
                Ordering::Greater
            });
        match pos {
            Err(pos) => {
                if pos > query_size {
                    -1
                } else {
                    pos as i32
                }
            }
            Ok(_) => -1, // 前面二分查找不会返回Equal，所以其实是不会进到这里的
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![2, 0, 2];
        let queries = vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]];
        assert_eq!(Solution::min_zero_array(nums, queries), 2);
    }

    #[test]
    fn test_case2() {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3, 2], vec![0, 2, 1]];
        assert_eq!(Solution::min_zero_array(nums, queries), -1);
    }

    #[test]
    fn test_case3() {
        let nums = vec![0, 0, 0, 0];
        let queries = vec![vec![1, 3, 2], vec![0, 2, 1]];
        assert_eq!(Solution::min_zero_array(nums, queries), 0);
    }
}
