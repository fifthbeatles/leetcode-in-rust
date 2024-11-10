/**
 * [208. Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/description/)
 */
use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    pub children: HashMap<char, Trie>,
    pub is_leaf_node: bool,
}

#[allow(dead_code)]
impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut current = self;
        for ch in word.chars() {
            let new_node = current.children.entry(ch).or_default();
            current = new_node;
        }
        current.is_leaf_node = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = self;
        for ch in word.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            }
        }
        current.is_leaf_node
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current = self;
        for ch in prefix.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
}
