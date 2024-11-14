/**
 * [54. Spiral Matrix](https://leetcode.com/problems/spiral-matrix/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut ret = Vec::with_capacity(rows * cols);

        let mut width = cols;
        let mut height = rows;

        let mut row = 0;
        let mut col = 0;
        loop {
            let size = width.min(height);
            match size {
                0 => break,
                1 => {
                    for _ in 1..width {
                        ret.push(matrix[row][col]);
                        col += 1;
                    }
                    for _ in 1..height {
                        ret.push(matrix[row][col]);
                        row += 1;
                    }
                    ret.push(matrix[row][col]);
                    break;
                }
                _ => {
                    for _ in 1..width {
                        ret.push(matrix[row][col]);
                        col += 1;
                    }
                    for _ in 1..height {
                        ret.push(matrix[row][col]);
                        row += 1;
                    }
                    for _ in 1..width {
                        ret.push(matrix[row][col]);
                        col -= 1;
                    }
                    for _ in 1..height {
                        ret.push(matrix[row][col]);
                        row -= 1;
                    }
                    width -= 2;
                    height -= 2;
                    col += 1;
                    row += 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test_case2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
