/**
 * [438. Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let p_bytes = p.into_bytes();
        let s_bytes = s.into_bytes();
        let p_size = p_bytes.len();
        let s_size = s_bytes.len();
        if p_size > s_bytes.len() {
            return vec![];
        }
        // 题目限制了全小写字母，要不就得用更大的数组或HashMap了
        let mut counts = [0; 26];
        for byte in &p_bytes {
            let idx = (*byte - b'a') as usize;
            counts[idx] += 1;
        }
        let mut window_counts = [0; 26];
        for byte in &s_bytes[0..p_size] {
            let idx = (*byte - b'a') as usize;
            if counts[idx] > 0 {
                window_counts[idx] += 1;
            }
        }
        let mut ret = vec![];
        if window_counts == counts {
            ret.push(0);
        }
        for i in 1..s_size - p_size + 1 {
            let old_char = s_bytes[i - 1];
            let old_idx = (old_char - b'a') as usize;
            let new_char = s_bytes[i - 1 + p_size];
            let new_idx = (new_char - b'a') as usize;
            if counts[old_idx] > 0 {
                window_counts[old_idx] -= 1;
            }
            if counts[new_idx] > 0 {
                window_counts[new_idx] += 1;
                if window_counts == counts {
                    ret.push(i as i32);
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
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::find_anagrams("baa".to_string(), "aa".to_string()),
            vec![1]
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::find_anagrams("aaaaaaaaaa".to_string(), "aaaaaaaaaaaaa".to_string()),
            vec![]
        );
    }
}
