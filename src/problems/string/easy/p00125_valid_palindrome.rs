/**
 * [125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        Self::is_palindrome_v1(s)
    }
    pub fn is_palindrome_v1(s: String) -> bool {
        let s1: Vec<u8> = s
            .to_ascii_lowercase()
            .bytes()
            .filter(|b| b.is_ascii_alphanumeric())
            .collect();
        let s2: Vec<u8> = s
            .to_ascii_lowercase()
            .bytes()
            .rev()
            .filter(|b| b.is_ascii_alphanumeric())
            .collect();
        s1 == s2
    }

    pub fn is_palindrome_v2(s: String) -> bool {
        let s1: Vec<u8> = s
            .to_ascii_lowercase()
            .bytes()
            .filter(|b| b.is_ascii_alphanumeric())
            .collect();
        let l = s1.len();
        for i in 0..l / 2 {
            if s1[i] != s1[l - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
