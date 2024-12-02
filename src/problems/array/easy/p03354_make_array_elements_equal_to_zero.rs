/**
 * [3354. Make Array Elements Equal to Zero](https://leetcode.com/problems/make-array-elements-equal-to-zero/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        Solution::count_valid_selections_v2(nums)
    }

    pub fn count_valid_selections_v1(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut tmp_prefix_sum = 0;
        let mut tmp_suffix_sum = 0;
        let mut prefix_sum = Vec::with_capacity(size + 1);
        prefix_sum.push(0);
        let mut suffix_sum = Vec::with_capacity(size + 1);
        suffix_sum.push(0);
        for idx in 0..size {
            tmp_prefix_sum += nums[idx];
            tmp_suffix_sum += nums[size - 1 - idx];
            prefix_sum.push(tmp_prefix_sum);
            suffix_sum.push(tmp_suffix_sum);
        }
        let mut selections = 0;
        for idx in 0..size {
            if nums[idx] != 0 {
                continue;
            }
            let left = prefix_sum[idx];
            let right = suffix_sum[size - 1 - idx];
            match left - right {
                0 => {
                    selections += 2;
                }
                1 | -1 => {
                    selections += 1;
                }
                _ => {
                    if left > right {
                        break;
                    }
                }
            }
        }
        selections
    }

    pub fn count_valid_selections_v2(nums: Vec<i32>) -> i32 {
        let size = nums.len();

        let mut tmp_prefix_sum = 0;
        let mut tmp_suffix_sum = 0;
        let mut prefix_sum = Vec::with_capacity(size + 1);
        prefix_sum.push(0);
        let mut suffix_sum = Vec::with_capacity(size + 1);
        suffix_sum.push(0);
        for idx in 0..size {
            tmp_prefix_sum += nums[idx];
            tmp_suffix_sum += nums[size - 1 - idx];
            prefix_sum.push(tmp_prefix_sum);
            suffix_sum.push(tmp_suffix_sum);
        }
        let mut selections = 0;
        let mut idx = 0;
        while idx < size {
            if nums[idx] != 0 {
                idx += 1;
                continue;
            }
            let left = prefix_sum[idx];
            let right = suffix_sum[size - 1 - idx];
            match left - right {
                0 => {
                    while idx < size && nums[idx] == 0 {
                        selections += 2;
                        idx += 1;
                    }
                    break;
                }
                -1 => {
                    while idx < size && nums[idx] == 0 {
                        selections += 1;
                        idx += 1;
                    }
                    if idx == size {
                        break;
                    }
                    idx += 1;
                }
                1 => {
                    while idx < size && nums[idx] == 0 {
                        selections += 1;
                        idx += 1;
                    }
                    break;
                }
                _ => {
                    if left > right {
                        break;
                    }
                    idx += 1;
                }
            }
        }
        selections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 0, 2, 0, 3];
        assert_eq!(Solution::count_valid_selections(nums), 2);
    }

    #[test]
    fn test_case2() {
        let nums = vec![2, 3, 4, 0, 4, 1, 0];
        assert_eq!(Solution::count_valid_selections(nums), 0);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 0, 1, 1, 2, 1, 0, 1, 0, 0, 0, 6];
        assert_eq!(Solution::count_valid_selections(nums), 4);
    }

    #[test]
    fn test_case4() {
        let nums = vec![0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Solution::count_valid_selections(nums), 16);
    }
}
