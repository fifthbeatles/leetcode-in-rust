/**
 * [973. K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        if k >= points.len() {
            return points;
        }
        points.select_nth_unstable_by(k, |a, b| {
            let va = a[0] * a[0] + a[1] * a[1];
            let vb = b[0] * b[0] + b[1] * b[1];
            va.cmp(&vb)
        });
        points[0..k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let v = Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        assert_eq!(v, vec![vec![-2, 2]]);
        let v = Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        assert!(v == vec![vec![3, 3], vec![-2, 4]] || v == vec![vec![-2, 4], vec![3, 3]]);
    }
}
