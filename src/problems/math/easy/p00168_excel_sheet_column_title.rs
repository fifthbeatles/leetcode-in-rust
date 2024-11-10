/**
 * [168. Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number - 1;
        let mut upper_bound = 26;
        let mut length: usize = 1;
        while n >= upper_bound {
            n -= upper_bound;
            upper_bound = upper_bound.saturating_mul(26);
            length += 1;
        }
        let mut chars = vec!['A'; length];
        for i in 0..length {
            chars[length - 1 - i] = Self::to_char(n % 26);
            n /= 26;
        }
        chars.iter().collect()
    }

    fn to_char(i: i32) -> char {
        (65 + i) as u8 as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
        assert_eq!(Solution::convert_to_title(703), "AAA".to_string());
        assert_eq!(Solution::convert_to_title(18279), "AAAA".to_string());
    }
}
