/**
 * [981. Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/description/)
 */
use std::collections::HashMap;

#[derive(Default)]
pub struct TimeMap {
    pub map: HashMap<String, Vec<(i32, String)>>,
}

#[allow(dead_code)]
impl TimeMap {
    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.get_option(key, timestamp).unwrap_or_default()
    }

    fn get_option(&self, key: String, timestamp: i32) -> Option<String> {
        let values = self.map.get(&key)?;
        match values.binary_search_by(|v| v.0.cmp(&timestamp)) {
            Ok(pos) => values.get(pos).map(|(_, v)| v.clone()),
            Err(pos) => {
                if pos == 0 {
                    None
                } else {
                    values.get(pos - 1).map(|(_, v)| v.clone())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 0), "".to_string());
    }
}
