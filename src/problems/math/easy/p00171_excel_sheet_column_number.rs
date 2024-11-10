/**
 * [171. Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        Self::title_to_number_v1(column_title)
    }
    pub fn title_to_number_v1(column_title: String) -> i32 {
        let mut column_number = 1;
        let mut pre_counts = 26;
        let length = column_title.len();
        for _ in 0..length - 1 {
            column_number += pre_counts;
            pre_counts *= 26;
        }
        let mut counts = 0;
        for c in column_title.chars() {
            counts = counts * 26 + (c as i32 - 65);
        }
        column_number += counts;
        column_number
    }

    pub fn title_to_number_v2(column_title: String) -> i32 {
        let mut column_number: i32 = 0;
        for c in column_title.bytes() {
            column_number *= 26;
            column_number += (c as i32 - 65) + 1;
        }
        column_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
        assert_eq!(Solution::title_to_number("AAA".to_string()), 703);
        assert_eq!(Solution::title_to_number("AAAA".to_string()), 18279);
    }
}
