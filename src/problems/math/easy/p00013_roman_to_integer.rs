/**
 * [13. Roman to Integer](https://leetcode.com/problems/roman-to-integer/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut pre_char = ' ';
        for c in s.chars() {
            match (c, pre_char) {
                ('M', 'C') => result += 800,
                ('M', _) => result += 1000,
                ('D', 'C') => result += 300,
                ('D', _) => result += 500,
                ('C', 'X') => result += 80,
                ('C', _) => result += 100,
                ('L', 'X') => result += 30,
                ('L', _) => result += 50,
                ('X', 'I') => result += 8,
                ('X', _) => result += 10,
                ('V', 'I') => result += 3,
                ('V', _) => result += 5,
                ('I', _) => result += 1,
                _ => {}
            }
            pre_char = c;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
