/**
 * [11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let size = height.len();
        let mut left = 0;
        let mut right = size - 1;
        let mut max_water = ((right - left) as i32) * height[left].min(height[right]);
        while left < right {
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
            let water = ((right - left) as i32) * height[left].min(height[right]);
            max_water = max_water.max(water);
        }
        max_water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn test_case2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
