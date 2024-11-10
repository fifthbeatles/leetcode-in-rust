/**
 * [57. Insert Interval](https://leetcode.com/problems/insert-interval/description/)
 */
pub struct Solution;

use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let left_pos = Self::binary_search_v2(&intervals, new_interval[0]);
        let right_pos = Self::binary_search_v2(&intervals, new_interval[1]);
        let (left, right, merged_interval) = match (left_pos, right_pos) {
            (Ok(pos1), Ok(pos2)) => (pos1, pos2 + 1, vec![intervals[pos1][0], intervals[pos2][1]]),
            (Err(pos1), Ok(pos2)) => (pos1, pos2 + 1, vec![new_interval[0], intervals[pos2][1]]),
            (Ok(pos1), Err(pos2)) => (pos1, pos2, vec![intervals[pos1][0], new_interval[1]]),
            (Err(pos1), Err(pos2)) => (pos1, pos2, vec![new_interval[0], new_interval[1]]),
        };
        intervals.splice(left..right, [merged_interval]);
        intervals
    }

    fn binary_search_v1(intervals: &[Vec<i32>], value: i32) -> Result<usize, usize> {
        intervals.binary_search_by(|interval| {
            if value < interval[0] {
                Ordering::Greater
            } else if value > interval[1] {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
    }

    /**
     * 尝试自己写二分查找
     */
    fn binary_search_v2(intervals: &[Vec<i32>], value: i32) -> Result<usize, usize> {
        let mut left: usize = 0;
        let mut right = intervals.len();
        while left < right {
            let mid = (left + right) / 2;
            if value < intervals[mid][0] {
                right = mid;
            } else if value > intervals[mid][1] {
                left = mid + 1;
            } else {
                return Ok(mid);
            }
        }
        Err(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16],
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
