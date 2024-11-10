/**
 * [150. Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        Solution::eval_rpn_option(tokens).unwrap()
    }

    pub fn eval_rpn_option(tokens: Vec<String>) -> Option<i32> {
        let mut stack = Vec::new();
        for token in tokens {
            if let Ok(int_value) = token.parse::<i32>() {
                stack.push(int_value);
            } else {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                let expr = if token == "+" {
                    Some(lhs + rhs)
                } else if token == "-" {
                    Some(lhs - rhs)
                } else if token == "*" {
                    Some(lhs * rhs)
                } else if token == "/" {
                    Some(lhs / rhs)
                } else {
                    None
                }?;
                stack.push(expr);
            }
        }
        if stack.len() != 1 {
            None
        } else {
            stack.pop()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let v = vec!["2", "1", "+", "3", "*"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        assert_eq!(Solution::eval_rpn(v), 9);
    }

    #[test]
    fn test_case2() {
        let v = vec!["4", "13", "5", "/", "+"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        assert_eq!(Solution::eval_rpn(v), 6);
    }

    #[test]
    fn test_case3() {
        let v = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        assert_eq!(Solution::eval_rpn(v), 22);
    }
}
