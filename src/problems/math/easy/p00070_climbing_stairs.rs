/**
 * [70. Climbing Stairs](https://leetcode.com/problems/climbing-stairs/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        for _ in 0..n - 1 {
            (a, b) = (b, a + b);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
