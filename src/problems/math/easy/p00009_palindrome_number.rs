/**
 * [9. Palindrome Number](https://leetcode.com/problems/palindrome-number/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut y = 0;
        let mut z = x;
        while z >= 10 {
            y = 10 * (y + z % 10);
            z /= 10;
        }
        y += z;
        x == y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
