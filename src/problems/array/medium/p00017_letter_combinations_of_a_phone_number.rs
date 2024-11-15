/**
 * [17. Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mapping = vec![
            vec![],
            vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut letters = vec![String::new()];
        for byte in digits.bytes() {
            let idx = (byte - b'0') as usize;
            if idx > 9 {
                continue;
            }
            let mut new_letters = vec![];
            for prefix in letters {
                for ch in &mapping[idx] {
                    let mut new_str = prefix.clone();
                    new_str.push(*ch);
                    new_letters.push(new_str);
                }
            }
            letters = new_letters;
        }
        letters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let digits = "23".to_string();
        let ret = Solution::letter_combinations(digits);
        let target = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        println!("{:?}", ret);
        assert_eq!(ret, target);
    }

    #[test]
    fn test_case2() {
        let digits = "".to_string();
        let ret = Solution::letter_combinations(digits);
        assert!(ret.is_empty());
    }

    #[test]
    fn test_case3() {
        let digits = "2".to_string();
        let ret = Solution::letter_combinations(digits);
        let target = vec!["a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        println!("{:?}", ret);
        assert_eq!(ret, target);
    }
}
