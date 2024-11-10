/**
 * [542. 01 Matrix](https://leetcode.com/problems/01-matrix/description/)
 */
pub struct Solution;

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut mat_distance = vec![vec![0; n]; m];
        let mut zeros = Vec::new();
        for row in 0..m {
            for col in 0..n {
                if mat[row][col] == 0 {
                    zeros.push((row as i32, col as i32));
                } else {
                    mat_distance[row][col] = (m + n) as i32;
                }
            }
        }
        if zeros.len() < 4 {
            // 0的数量很少时，就以0的位置出发算各点的距离，4是根据比较复杂度确定的
            for row in 0..m {
                for col in 0..n {
                    if mat[row][col] == 0 {
                        continue;
                    }
                    for (x, y) in &zeros {
                        mat_distance[row][col] = mat_distance[row][col]
                            .min((*x - row as i32).abs() + (*y - col as i32).abs());
                    }
                }
            }
        } else {
            // 下面的遍历要注意比较顺序，例如从左往右遍历，就要不断的和左边的点比较，其他方向同理
            for row in 0..m {
                for col in (0..n - 1).rev() {
                    if mat_distance[row][col + 1] + 1 < mat_distance[row][col] {
                        mat_distance[row][col] = mat_distance[row][col + 1] + 1;
                    }
                }
                for col in 1..n {
                    if mat_distance[row][col - 1] + 1 < mat_distance[row][col] {
                        mat_distance[row][col] = mat_distance[row][col - 1] + 1;
                    }
                }
            }
            for col in 0..n {
                for row in (0..m - 1).rev() {
                    if mat_distance[row + 1][col] + 1 < mat_distance[row][col] {
                        mat_distance[row][col] = mat_distance[row + 1][col] + 1;
                    }
                }
                for row in 1..m {
                    if mat_distance[row - 1][col] + 1 < mat_distance[row][col] {
                        mat_distance[row][col] = mat_distance[row - 1][col] + 1;
                    }
                }
            }
        }
        mat_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let mat_distance = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::update_matrix(mat), mat_distance);
    }

    #[test]
    fn test_case2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let mat_distance = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        assert_eq!(Solution::update_matrix(mat), mat_distance);
    }

    #[test]
    fn test_case3() {
        let mat = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 0, 1]];
        let mat_distance = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::update_matrix(mat), mat_distance);
    }
}
