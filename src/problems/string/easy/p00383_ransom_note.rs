/**
 * [383. Ransom Note](https://leetcode.com/problems/ransom-note/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        Self::can_construct_v2(ransom_note, magazine)
    }
    pub fn can_construct_v1(ransom_note: String, magazine: String) -> bool {
        let mut letters = HashMap::new();
        magazine.chars().for_each(|c| {
            letters
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
        for c in ransom_note.chars() {
            match letters.get_mut(&c) {
                Some(count) if *count > 0 => {
                    *count -= 1;
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }

    pub fn can_construct_v2(ransom_note: String, magazine: String) -> bool {
        let mut letters = [0; 26];
        for c in magazine.chars() {
            letters[(c as u8 - 97) as usize] += 1;
        }
        for c in ransom_note.chars() {
            let pos = (c as u8 - 97) as usize;
            if letters[pos] <= 0 {
                return false;
            }
            letters[pos] -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::can_construct("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "ab".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "aab".to_string()),
            true
        );
    }
}
