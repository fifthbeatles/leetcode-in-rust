/**
 * [297. Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/)
 */
use crate::data_structures::binary_tree::{from_vec_str, to_vec, TreeNode};

pub struct Codec {}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        to_vec(&root)
            .iter()
            .map(|v| {
                if v.is_none() {
                    "null".to_string()
                } else {
                    v.unwrap().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        from_vec_str(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let codec = Codec::new();
        assert_eq!(
            codec.serialize(codec.deserialize("1,2,3,null,null,4,5".to_string())),
            "1,2,3,null,null,4,5".to_string()
        );
    }

    #[test]
    fn test_case2() {
        let codec = Codec::new();
        assert_eq!(
            codec.serialize(codec.deserialize("".to_string())),
            "".to_string()
        );
    }
}
