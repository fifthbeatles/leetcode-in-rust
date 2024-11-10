/**
 * [226. Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::{level_order_traversal_fn, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        level_order_traversal_fn(&root, |node, _| {
            (node.borrow_mut().left, node.borrow_mut().right) =
                (node.borrow().right.clone(), node.borrow().left.clone());
            false
        });
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::{from_vec_str, to_vec};

    #[test]
    fn test_case1() {
        let root = from_vec_str("4,2,7,1,3,6,9");
        let new_root = Solution::invert_tree(root);
        let v = to_vec(&new_root);
        assert_eq!(
            v,
            vec![
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1),
            ]
        );
        println!("{:?}", v);
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("2,1,3");
        let new_root = Solution::invert_tree(root);
        let v = to_vec(&new_root);
        assert_eq!(v, vec![Some(2), Some(3), Some(1)]);
        println!("{:?}", v);
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        let new_root = Solution::invert_tree(root);
        let v = to_vec(&new_root);
        assert!(v.is_empty());
    }
}
