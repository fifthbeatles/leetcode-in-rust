/**
 * [56. Merge Intervals](https://leetcode.com/problems/merge-intervals/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::merge_v2(intervals)
    }

    pub fn merge_v1(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut new_intervals = Vec::new();
        if let Some(first) = intervals.first() {
            new_intervals.push(first.clone());
            for interval in intervals.iter().skip(1) {
                let pre_interval = new_intervals.last_mut().unwrap();
                if interval[0] <= pre_interval[1] {
                    pre_interval[1] = interval[1].max(pre_interval[1]);
                } else {
                    new_intervals.push(interval.clone());
                }
            }
        }
        new_intervals
    }

    pub fn merge_v2(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<_> = intervals.iter().map(|v| (v[0], v[1])).collect();
        intervals.sort();
        let mut new_intervals = Vec::new();
        if let Some(first) = intervals.first() {
            new_intervals.push(vec![first.0, first.1]);
            for interval in intervals.iter().skip(1) {
                let pre_interval = new_intervals.last_mut().unwrap();
                if interval.0 <= pre_interval[1] {
                    pre_interval[1] = interval.1.max(pre_interval[1]);
                } else {
                    new_intervals.push(vec![interval.0, interval.1]);
                }
            }
        }
        new_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ret = Solution::merge(intervals);
        assert_eq!(ret, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_case2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let ret = Solution::merge(intervals);
        assert_eq!(ret, vec![vec![1, 5]]);
    }

    #[test]
    fn test_case3() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        let ret = Solution::merge(intervals);
        assert_eq!(ret, vec![vec![1, 4]]);
    }
}
