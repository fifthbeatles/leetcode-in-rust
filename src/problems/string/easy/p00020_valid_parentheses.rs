/**
 * [20. Valid Parentheses](https://leetcode.com/problems/valid-parentheses/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                ']' | '}' | ')' => {
                    if Some(c) != stack.pop() {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([])".to_string()), true);
    }
}
