/**
 * [190. Reverse Bits](https://leetcode.com/problems/reverse-bits/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }

    pub fn reverse_bits_str(x: String) -> u32 {
        let y: String = x.chars().rev().collect();
        u32::from_str_radix(&y, 2).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
    }

    #[test]
    fn test_case_str() {
        assert_eq!(
            Solution::reverse_bits_str("00000010100101000001111010011100".to_string()),
            964176192
        );
        assert_eq!(
            Solution::reverse_bits_str("11111111111111111111111111111101".to_string()),
            3221225471
        );
    }
}
