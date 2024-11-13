/**
 * [994. Rotting Oranges](https://leetcode.com/problems/rotting-oranges/description/)
 */
pub struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        Solution::oranges_rotting_v1(grid)
    }

    /**
     * 从前一轮腐烂的为起点找下一轮
     */
    pub fn oranges_rotting_v1(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut minutes = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue1 = VecDeque::new();
        let mut to_rotting = 0;
        for (row, row_item) in grid.iter().enumerate() {
            for (col, &value) in row_item.iter().enumerate() {
                if value == 1 {
                    to_rotting += 1;
                } else if value == 2 {
                    queue1.push_back((row, col));
                }
            }
        }
        while !queue1.is_empty() {
            let mut queue2 = VecDeque::new();
            while let Some((row, col)) = queue1.pop_front() {
                if row > 0 && grid[row - 1][col] == 1 {
                    queue2.push_back((row - 1, col));
                    grid[row - 1][col] = 2;
                }
                if row < rows - 1 && grid[row + 1][col] == 1 {
                    queue2.push_back((row + 1, col));
                    grid[row + 1][col] = 2;
                }
                if col > 0 && grid[row][col - 1] == 1 {
                    queue2.push_back((row, col - 1));
                    grid[row][col - 1] = 2;
                }
                if col < cols - 1 && grid[row][col + 1] == 1 {
                    queue2.push_back((row, col + 1));
                    grid[row][col + 1] = 2;
                }
            }
            if queue2.is_empty() {
                break;
            } else {
                minutes += 1;
                to_rotting -= queue2.len();
                queue1 = queue2;
            }
        }
        if to_rotting > 0 {
            -1
        } else {
            minutes
        }
    }

    /**
     * 从所有待腐烂为候选
     */
    pub fn oranges_rotting_v2(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut minutes = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue1 = VecDeque::with_capacity(rows * cols);
        for (row, row_item) in grid.iter().enumerate() {
            for (col, &value) in row_item.iter().enumerate() {
                if value == 1 {
                    queue1.push_back((row, col));
                }
            }
        }
        while !queue1.is_empty() {
            minutes += 1;
            let before_counts = queue1.len();
            let mut queue2 = VecDeque::new();
            let mut queue3 = VecDeque::new();
            while let Some((row, col)) = queue1.pop_front() {
                if row > 0 && grid[row - 1][col] == 2 {
                    queue3.push_back((row, col));
                    continue;
                }
                if row < rows - 1 && grid[row + 1][col] == 2 {
                    queue3.push_back((row, col));
                    continue;
                }
                if col > 0 && grid[row][col - 1] == 2 {
                    queue3.push_back((row, col));
                    continue;
                }
                if col < cols - 1 && grid[row][col + 1] == 2 {
                    queue3.push_back((row, col));
                    continue;
                }
                queue2.push_back((row, col));
            }
            while let Some((row, col)) = queue3.pop_front() {
                grid[row][col] = 2;
            }
            let after_counts = queue2.len();
            if after_counts == 0 {
                break;
            }
            if after_counts == before_counts {
                return -1;
            }
            queue1 = queue2;
        }
        minutes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4);
    }

    #[test]
    fn test_case2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(grid), -1);
    }

    #[test]
    fn test_case3() {
        let grid = vec![vec![0, 2]];
        assert_eq!(Solution::oranges_rotting(grid), 0);
    }
}
