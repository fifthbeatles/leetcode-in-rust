/**
 * [42. Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        Solution::trap_v2(height)
    }

    pub fn trap_v1(height: Vec<i32>) -> i32 {
        let size = height.len();
        if size <= 2 {
            return 0;
        }
        let mut max_height = height[0];
        let mut max_pos = 0;
        for (idx, &h) in height.iter().enumerate() {
            if h > max_height {
                max_height = h;
                max_pos = idx;
            }
        }
        Solution::trap_from_left(&height[0..max_pos + 1])
            + Solution::trap_from_right(&height[max_pos..size])
    }

    /**
     * 最后一个是最高的
     */
    fn trap_from_left(height: &[i32]) -> i32 {
        let size = height.len();
        let mut left = 0;
        // 第一个非0的
        while left < size && height[left] == 0 {
            left += 1;
        }
        let mut water = 0;
        while left < size {
            let mut right = left + 1;
            // 找到下一个不低于left的位置
            while right < size && height[right] < height[left] {
                right += 1;
            }
            if right == size {
                break;
            }
            for i in left + 1..right {
                water += height[left] - height[i];
            }
            left = right;
        }
        water
    }

    /**
     * 第一个是最高的
     */
    fn trap_from_right(height: &[i32]) -> i32 {
        let size = height.len();
        let mut left = 0;
        // 如果直接从右向左移动，下标是usize处理0有点麻烦，所以用size-1-left替代
        while left < size && height[size - 1 - left] == 0 {
            left += 1;
        }
        let mut water = 0;
        while left < size {
            let mut right = left + 1;
            while right < size && height[size - 1 - right] < height[size - 1 - left] {
                right += 1;
            }
            if right == size {
                break;
            }
            for i in left + 1..right {
                water += height[size - 1 - left] - height[size - 1 - i];
            }
            left = right;
        }
        water
    }

    pub fn trap_v2(height: Vec<i32>) -> i32 {
        let mut left = 0_usize;
        let mut right = height.len() - 1;
        let mut max_left = 0;
        let mut max_right = 0;
        let mut water = 0;
        while left <= right {
            if height[left] <= height[right] {
                if height[left] >= max_left {
                    max_left = height[left];
                    left += 1;
                } else {
                    water += max_left - height[left];
                    left += 1;
                }
            } else if height[right] >= max_right {
                max_right = height[right];
                right -= 1;
            } else {
                water += max_right - height[right];
                right -= 1;
            }
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn test_case2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }
}
