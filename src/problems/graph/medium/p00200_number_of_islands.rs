/**
 * [200. Number of Islands](https://leetcode.com/problems/number-of-islands/description/)
 */
pub struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue = VecDeque::with_capacity(rows * cols);
        let mut visited = vec![vec![false; cols]; rows];
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '1' || visited[row][col] {
                    continue;
                }
                islands += 1;
                queue.push_back((row, col));
                // 这里注意要在入队时就更新visited，否则同一个点可能被入队不止1次，LeetCode上会超内存报错
                visited[row][col] = true;
                while let Some((r, c)) = queue.pop_front() {
                    if r > 0 && grid[r - 1][c] == '1' && !visited[r - 1][c] {
                        queue.push_back((r - 1, c));
                        visited[r - 1][c] = true;
                    }

                    if r < rows - 1 && grid[r + 1][c] == '1' && !visited[r + 1][c] {
                        queue.push_back((r + 1, c));
                        visited[r + 1][c] = true;
                    }

                    if c > 0 && grid[r][c - 1] == '1' && !visited[r][c - 1] {
                        queue.push_back((r, c - 1));
                        visited[r][c - 1] = true;
                    }

                    if c < cols - 1 && grid[r][c + 1] == '1' && !visited[r][c + 1] {
                        queue.push_back((r, c + 1));
                        visited[r][c + 1] = true;
                    }
                }
            }
        }
        islands
    }

    pub fn num_islands_mut(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut stack = Vec::with_capacity(rows * cols);

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '1' {
                    continue;
                }
                islands += 1;
                stack.push((row, col));
                grid[row][col] = '0';
                while let Some((r, c)) = stack.pop() {
                    if r > 0 && grid[r - 1][c] == '1' {
                        stack.push((r - 1, c));
                        grid[r - 1][c] = '0';
                    }

                    if r < rows - 1 && grid[r + 1][c] == '1' {
                        stack.push((r + 1, c));
                        grid[r + 1][c] = '0';
                    }

                    if c > 0 && grid[r][c - 1] == '1' {
                        stack.push((r, c - 1));
                        grid[r][c - 1] = '0';
                    }

                    if c < cols - 1 && grid[r][c + 1] == '1' {
                        stack.push((r, c + 1));
                        grid[r][c + 1] = '0';
                    }
                }
            }
        }
        islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_case2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_case3() {
        let grid = vec![vec!['1']];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_case4() {
        let grid = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }
}
