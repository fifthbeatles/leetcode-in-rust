/**
 * [84. Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/description/)
 */
pub struct Solution;

use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        Solution::largest_rectangle_area_v2(heights)
    }

    pub fn largest_rectangle_area_v1(heights: Vec<i32>) -> i32 {
        let size = heights.len();
        let mut left_boundaries = vec![0; size];
        for i in 1..size {
            let height = heights[i];
            match height.cmp(&heights[i - 1]) {
                Ordering::Greater => left_boundaries[i] = i,
                Ordering::Less => {
                    let mut left = left_boundaries[i - 1];
                    for j in (0..left_boundaries[i - 1]).rev() {
                        if heights[j] < height {
                            break;
                        }
                        left = j;
                    }
                    left_boundaries[i] = left;
                }
                Ordering::Equal => left_boundaries[i] = left_boundaries[i - 1],
            }
        }
        let mut right_boundaries = vec![0; size];
        right_boundaries[size - 1] = size - 1;
        let mut largest_area =
            heights[size - 1] * (1 + right_boundaries[size - 1] - left_boundaries[size - 1]) as i32;
        for i in (0..size - 1).rev() {
            let height = heights[i];
            match height.cmp(&heights[i + 1]) {
                Ordering::Greater => right_boundaries[i] = i,
                Ordering::Less => {
                    let mut right = right_boundaries[i + 1];
                    #[allow(clippy::needless_range_loop)]
                    for j in right_boundaries[i + 1] + 1..size {
                        if heights[j] < height {
                            break;
                        }
                        right = j;
                    }
                    right_boundaries[i] = right;
                }
                Ordering::Equal => right_boundaries[i] = right_boundaries[i + 1],
            }
            largest_area =
                largest_area.max(height * (1 + right_boundaries[i] - left_boundaries[i]) as i32);
        }
        largest_area
    }

    /**
     * 单调栈方法
     */
    pub fn largest_rectangle_area_v2(arr: Vec<i32>) -> i32 {
        let prev = Self::prev_index(&arr);
        let next = Self::next_index(&arr);
        println!("{:?}", prev);
        println!("{:?}", next);
        let mut largest_area = 0;
        for i in 0..arr.len() {
            largest_area = largest_area.max(arr[i] * (next[i] - prev[i] + 1) as i32);
        }
        largest_area
    }

    fn prev_index(arr: &[i32]) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut ret = vec![0; arr.len()];
        for i in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.last().unwrap()] >= arr[i] {
                stack.pop();
            }
            if stack.is_empty() {
                ret[i] = 0;
            } else {
                ret[i] = stack.last().unwrap() + 1;
            }
            stack.push(i);
        }
        ret
    }

    fn next_index(arr: &[i32]) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut ret = vec![0; arr.len()];
        for i in (0..arr.len()).rev() {
            while !stack.is_empty() && arr[*stack.last().unwrap()] >= arr[i] {
                stack.pop();
            }
            if stack.is_empty() {
                ret[i] = arr.len() - 1;
            } else {
                ret[i] = stack.last().unwrap() - 1;
            }
            stack.push(i);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let height = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(Solution::largest_rectangle_area(height), 10);
    }

    #[test]
    fn test_case2() {
        let height = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(height), 4);
    }

    #[test]
    fn test_case3() {
        let height = vec![2, 1, 2];
        assert_eq!(Solution::largest_rectangle_area(height), 3);
    }
}
