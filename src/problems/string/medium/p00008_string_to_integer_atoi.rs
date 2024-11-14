/**
 * [8. String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut ret = 0;
        let mut started = false;
        let mut positive = true;
        for ch in s.chars() {
            if started {
                match ch {
                    '0'..='9' => {
                        if positive {
                            let v = (ch as i32) - 48;
                            if ret > 214748364 || (ret == 214748364 && v > 7) {
                                ret = i32::MAX;
                                break;
                            }
                            ret = ret * 10 + v;
                        } else {
                            let v = 48 - (ch as i32);
                            if ret < -214748364 || (ret == -214748364) && v < -8 {
                                ret = i32::MIN;
                                break;
                            }
                            ret = ret * 10 + v;
                        }
                    }
                    _ => break,
                }
            } else {
                match ch {
                    ' ' => continue,
                    '+' => {
                        started = true;
                    }
                    '-' => {
                        started = true;
                        positive = false;
                    }
                    '0'..='9' => {
                        started = true;
                        let v = (ch as i32) - 48;
                        ret = ret * 10 + v;
                    }
                    _ => break,
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::my_atoi("-042".to_string()), -42);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    }

    #[test]
    fn test_case5() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
