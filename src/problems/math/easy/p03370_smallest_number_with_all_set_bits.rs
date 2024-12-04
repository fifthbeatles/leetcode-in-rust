/**
 * [3370. Smallest Number With All Set Bits](https://leetcode.com/problems/smallest-number-with-all-set-bits/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut ret = 1;
        while n > ret - 1 {
            ret <<= 1;
        }
        ret - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::smallest_number(5), 7);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::smallest_number(10), 15);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::smallest_number(3), 3);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::smallest_number(2), 3);
    }

    #[test]
    fn test_case5() {
        assert_eq!(Solution::smallest_number(1), 1);
    }
}
