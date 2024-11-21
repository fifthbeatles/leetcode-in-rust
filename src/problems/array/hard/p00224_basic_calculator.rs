/**
 * [224. Basic Calculator](https://leetcode.com/problems/basic-calculator/description/)
 */
pub struct Solution;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Negative,
    BracketStart,
}

#[allow(dead_code)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        for byte in s.bytes() {
            match byte {
                b'+' => {
                    stack.push(Token::Plus);
                }
                b'-' => match stack.last() {
                    Some(Token::Number(_)) => stack.push(Token::Minus),
                    _ => stack.push(Token::Negative),
                },
                b'(' => {
                    stack.push(Token::BracketStart);
                }
                b')' => {
                    let mut tmp_stack = Vec::new();
                    while let Some(token) = stack.pop() {
                        if let Token::BracketStart = token {
                            break;
                        }
                        tmp_stack.push(token);
                    }
                    let mut value = 0;
                    if let Some(Token::Number(number)) = tmp_stack.pop() {
                        value = number;
                    }
                    let mut pre_token = Token::Number(value);
                    while let Some(token) = tmp_stack.pop() {
                        match (token, pre_token) {
                            (Token::Number(number), Token::Plus) => {
                                value += number;
                            }
                            (Token::Number(number), Token::Minus) => {
                                value -= number;
                            }
                            _ => {}
                        }
                        pre_token = token;
                    }
                    if let Some(Token::Negative) = stack.last() {
                        stack.pop();
                        stack.push(Token::Number(-value));
                    } else {
                        stack.push(Token::Number(value));
                    }
                }
                b'0'..=b'9' => {
                    let int_value = (byte - b'0') as i32;
                    match stack.last() {
                        Some(&Token::Number(number)) => {
                            stack.pop();
                            if number > 0 {
                                stack.push(Token::Number(number * 10 + int_value));
                            } else {
                                stack.push(Token::Number(number * 10 - int_value));
                            }
                        }
                        Some(Token::Negative) => {
                            stack.pop();
                            stack.push(Token::Number(-int_value));
                        }
                        _ => {
                            stack.push(Token::Number(int_value));
                        }
                    }
                }
                _ => {}
            }
        }
        let mut value = 0;
        if let Some(&Token::Number(number)) = stack.first() {
            value = number;
        }
        let mut pre_token = Token::Number(value);
        for &token in stack.iter().skip(1) {
            match (token, pre_token) {
                (Token::Number(number), Token::Plus) => {
                    value += number;
                }
                (Token::Number(number), Token::Minus) => {
                    value -= number;
                }
                _ => {}
            }
            pre_token = token;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::calculate("1-(-(6+8))".to_string()), 15);
    }
}
