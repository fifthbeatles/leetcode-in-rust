/**
 * [102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::level_order_traversal_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        level_order_traversal_fn(&root, |node, level| {
            if let Some(vec) = ret.get_mut(level as usize) {
                vec.push(node.borrow().val);
            } else {
                ret.push(vec![node.borrow().val]);
            }
            false
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::from_vec_str;

    #[test]
    fn test_case1() {
        let root = from_vec_str("3,9,20,null,null,15,7");
        assert_eq!(
            Solution::level_order(root),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("1");
        assert_eq!(Solution::level_order(root), vec![vec![1]]);
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        assert_eq!(Solution::level_order(root).is_empty(), true);
    }
}
