/**
 * [409. Longest Palindrome](https://leetcode.com/problems/longest-palindrome/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts = vec![0; 127];
        for c in s.chars() {
            let pos = c as usize;
            counts[pos] += 1;
        }
        let mut has_odd = false;
        let mut length = 0;
        for c in counts {
            if c % 2 != 0 {
                has_odd = true;
                length += c - 1;
            } else {
                length += c;
            }
        }
        if has_odd {
            length + 1
        } else {
            length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
        assert_eq!(Solution::longest_palindrome("ccccdd".to_string()), 6);
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }
}
