/**
 * [3364. Minimum Positive Sum Subarray](https://leetcode.com/problems/minimum-positive-sum-subarray/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        Solution::minimum_sum_subarray_v3(nums, l, r)
    }

    /**
     * 比较直接的解法，利用了标准库的windows方法实现窗口滑动
     */
    pub fn minimum_sum_subarray_v1(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ret = i32::MAX;
        for size in l..=r {
            for w in nums.windows(size as usize) {
                let s = w.iter().sum();
                if s > 0 && s < ret {
                    ret = s;
                }
            }
        }
        if ret == i32::MAX {
            -1
        } else {
            ret
        }
    }

    /**
     * 在滑动窗口的基础上，基于前一次窗口计算的和快速计算下一个窗口
     */
    pub fn minimum_sum_subarray_v2(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ret = i32::MAX;
        // 求前l-1个数的和
        let mut init_s: i32 = nums.iter().take(l as usize - 1).sum();
        for size in l..=r {
            let mut it = nums.windows(size as usize);
            let w = it.next().unwrap();
            init_s += w.last().unwrap();
            let mut s = init_s;
            if s > 0 && s < ret {
                ret = s;
            }
            let mut to_sub = w.first().unwrap();
            for w in it {
                s = s + w.last().unwrap() - to_sub;
                to_sub = w.first().unwrap();
                if s > 0 && s < ret {
                    ret = s;
                }
            }
        }
        if ret == i32::MAX {
            -1
        } else {
            ret
        }
    }

    /**
     * 利用预计算前缀和快速计算各窗口的和，比前面的方法更简洁
     */
    pub fn minimum_sum_subarray_v3(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let size = nums.len();
        let l = l as usize;
        let r = r as usize;
        // 相当于前面多加了一个0
        let mut prefix_sum = vec![0; size + 1];
        let mut tmp_sum = 0;
        for (idx, &v) in nums.iter().enumerate() {
            tmp_sum += v;
            prefix_sum[idx + 1] = tmp_sum;
        }
        let mut ret = i32::MAX;
        for step in l..=r {
            for idx in 0..=size - step {
                let s = prefix_sum[idx + step] - prefix_sum[idx];
                if s > 0 && s < ret {
                    ret = s;
                }
            }
        }
        if ret == i32::MAX {
            -1
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![3, -2, 1, 4];
        assert_eq!(Solution::minimum_sum_subarray(nums, 2, 3), 1);
    }

    #[test]
    fn test_case2() {
        let nums = vec![-2, 2, -3, 1];
        assert_eq!(Solution::minimum_sum_subarray(nums, 2, 3), -1);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::minimum_sum_subarray(nums, 2, 4), 3);
    }
}
