/**
 * [733. Flood Fill](https://leetcode.com/problems/flood-fill/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let ori_color = image[sr as usize][sc as usize];
        if ori_color == color {
            return image;
        }
        let rows = image.len();
        let cols = &image[0].len();

        let sr = sr as usize;
        let sc = sc as usize;
        image[sr][sc] = color;
        let mut stack = vec![(sr, sc)];
        while let Some((row, col)) = stack.pop() {
            image[row][col] = color;
            if row > 0 && image[row - 1][col] == ori_color {
                stack.push((row - 1, col));
            }
            if row < rows - 1 && image[row + 1][col] == ori_color {
                stack.push((row + 1, col));
            }
            if col > 0 && image[row][col - 1] == ori_color {
                stack.push((row, col - 1));
            }
            if col < cols - 1 && image[row][col + 1] == ori_color {
                stack.push((row, col + 1));
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution::flood_fill(image, 1, 1, 2), expected);
    }

    #[test]
    fn test_case2() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::flood_fill(image, 0, 0, 0), expected);
    }
}
