/**
 * [75. Sort Colors](https://leetcode.com/problems/sort-colors/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        Solution::sort_colors_v2(nums)
    }

    pub fn sort_colors_v1(nums: &mut [i32]) {
        let mut counts = [0, 0, 0];
        for &color in nums.iter() {
            counts[color as usize] += 1;
        }
        let mut idx = 0;
        for (color, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                nums[idx] = color as i32;
                idx += 1;
            }
        }
    }

    pub fn sort_colors_v2(nums: &mut [i32]) {
        let mut p0 = 0;
        let mut p2 = nums.len() - 1;
        let mut i = 0;
        while i <= p2 {
            match nums[i] {
                0 => {
                    nums.swap(i, p0);
                    p0 += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, p2);
                    if p2 == 0 {
                        break;
                    }
                    p2 -= 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
