/**
 * [127. Word Ladder](https://leetcode.com/problems/word-ladder/description/)
 */
pub struct Solution;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let size = word_list.len();
        let mut costs = vec![0; size];
        let mut end_word_pos = size;
        let mut queue = VecDeque::new();
        let mut cost = 1;
        let mut word_idx_set = (0..size).collect::<HashSet<_>>();
        for (idx, s1) in word_list.iter().enumerate() {
            if end_word_pos == size && *s1 == end_word {
                end_word_pos = idx;
            }
            let diff_count = s1
                .bytes()
                .zip(begin_word.bytes())
                .filter(|(b1, b2)| *b1 != *b2)
                .count();
            if diff_count == 1 {
                queue.push_back(idx);
                costs[idx] = cost;
                word_idx_set.remove(&idx);
            }
        }
        if end_word_pos == size {
            return 0;
        }
        while !queue.is_empty() && costs[end_word_pos] == 0 {
            let pre_size = queue.len();
            cost += 1;
            for _ in 0..pre_size {
                let idx1 = queue.pop_front().unwrap();
                let mut tmp_idx = Vec::new();
                for &idx2 in &word_idx_set {
                    let diff_count = word_list[idx1]
                        .bytes()
                        .zip(word_list[idx2].bytes())
                        .filter(|(b1, b2)| *b1 != *b2)
                        .count();
                    if diff_count == 1 {
                        queue.push_back(idx2);
                        tmp_idx.push(idx2);
                    }
                }
                for idx in tmp_idx {
                    costs[idx] = cost;
                    word_idx_set.remove(&idx);
                }
            }
        }
        if costs[end_word_pos] > 0 {
            costs[end_word_pos] + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    }

    #[test]
    fn test_case2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }

    #[test]
    fn test_case3() {
        let begin_word = "hot".to_string();
        let end_word = "dog".to_string();
        let word_list = vec!["hot", "dog"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }

    #[test]
    fn test_case4() {
        let begin_word = "ymain".to_string();
        let end_word = "oecij".to_string();
        let word_list = vec![
            "ymann", "yycrj", "oecij", "ymcnj", "yzcrj", "yycij", "xecij", "yecij", "ymanj",
            "yzcnj", "ymain",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 10);
    }

    #[test]
    fn test_case5() {
        let begin_word = "a".to_string();
        let end_word = "c".to_string();
        let word_list = vec!["a", "b", "c"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 2);
    }
}
