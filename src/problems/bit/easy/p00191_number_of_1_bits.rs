/**
 * [191. Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut c = 0;
        while n > 0 {
            if n % 2 == 1 {
                c += 1;
            }
            n >>= 1;
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::hamming_weight(11), 3);
        assert_eq!(Solution::hamming_weight(128), 1);
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
