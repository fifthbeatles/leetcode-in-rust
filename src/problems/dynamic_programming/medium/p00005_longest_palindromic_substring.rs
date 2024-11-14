/**
 * [5. Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Solution::longest_palindrome_v2(s)
    }

    pub fn longest_palindrome_v1(s: String) -> String {
        let bytes = s.bytes().collect::<Vec<_>>();
        let size = bytes.len();
        let mut dp = vec![vec![false; size]; size];
        dp[0][0] = true;
        let mut max_length = 1;
        let mut max_pos = (0, 0);
        for idx in 1..size {
            dp[idx][idx] = true;
            // 连续相同字母
            if bytes[idx] == bytes[idx - 1] {
                dp[idx - 1][idx] = true;
                if max_length < 2 {
                    max_length = 2;
                    max_pos = (idx - 1, idx);
                }
            }
            // 尝试基于前面的序列扩展
            for i in 1..idx {
                if dp[i][idx - 1] && bytes[i - 1] == bytes[idx] {
                    dp[i - 1][idx] = true;
                    if idx + 2 - i > max_length {
                        max_length = idx + 2 - i;
                        max_pos = (i - 1, idx);
                    }
                }
            }
        }
        bytes[max_pos.0..=max_pos.1]
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    }

    pub fn longest_palindrome_v2(s: String) -> String {
        let bytes = s.bytes().collect::<Vec<_>>();
        let size = bytes.len();
        let mut max_length = 1;
        let mut max_pos = (0, 0);
        // 长度为奇数的情况
        for mid in 1..size - 1 {
            let width = mid.min(size - 1 - mid);
            let mut i = 0;
            while i < width {
                if bytes[mid - i - 1] != bytes[mid + i + 1] {
                    break;
                }
                i += 1;
            }
            if i >= 1 && max_length < 2 * i + 1 {
                max_length = 2 * i + 1;
                max_pos = (mid - i, mid + i);
            }
        }
        // 长度为偶数的情况，即中间两个字母一样
        for mid in 0..size - 1 {
            if bytes[mid] != bytes[mid + 1] {
                continue;
            }
            if max_length < 2 {
                max_length = 2;
                max_pos = (mid, mid + 1);
            }
            let width = mid.min(size - 2 - mid);
            let mut i = 0;
            while i < width {
                if bytes[mid - i - 1] != bytes[mid + i + 2] {
                    break;
                }
                i += 1;
            }
            if i >= 1 && max_length < 2 * i + 2 {
                max_length = 2 * i + 2;
                max_pos = (mid - i, mid + i + 1);
            }
        }
        bytes[max_pos.0..=max_pos.1]
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::longest_palindrome("ccc".to_string()),
            "ccc".to_string()
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::longest_palindrome("cccc".to_string()),
            "cccc".to_string()
        );
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn test_case6() {
        assert_eq!(
            Solution::longest_palindrome("bananas".to_string()),
            "anana".to_string()
        );
    }

    #[test]
    fn test_case7() {
        assert_eq!(
            Solution::longest_palindrome("ababababababa".to_string()),
            "ababababababa".to_string()
        );
    }
}
