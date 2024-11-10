/**
 * [543. Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::postorder_non_recursive_fn;
use crate::data_structures::binary_tree::postorder_recursive_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter_of_binary_tree_v1(root)
    }

    pub fn diameter_of_binary_tree_v1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ret = postorder_non_recursive_fn(
            &root,
            &mut |_| (1, 0),
            &mut || (0, 0),
            &mut |left, right, _| {
                (
                    false, // 一直算到根节点
                    (
                        left.0.max(right.0) + 1,                     // 高度
                        (left.0 + right.0).max(left.1).max(right.1), // diameter
                    ),
                )
            },
        );
        ret.1
    }

    pub fn diameter_of_binary_tree_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ret = postorder_recursive_fn::<(i32, i32)>(
            &root,
            &mut |left, right, _| {
                (
                    left.0.max(right.0) + 1,                     // 高度
                    (left.0 + right.0).max(left.1).max(right.1), // diameter
                )
            },
            &mut || (0, 0),
        );
        ret.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::{from_vec_str, to_vec};

    #[test]
    fn test_case1() {
        let root = from_vec_str("1,2,3,4,5");
        assert_eq!(Solution::diameter_of_binary_tree_v1(root.clone()), 3);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::diameter_of_binary_tree_v2(root.clone()), 3);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("1,2");
        assert_eq!(Solution::diameter_of_binary_tree_v1(root.clone()), 1);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::diameter_of_binary_tree_v2(root.clone()), 1);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        assert_eq!(Solution::diameter_of_binary_tree_v1(root.clone()), 0);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::diameter_of_binary_tree_v2(root.clone()), 0);
        println!("{:?}", to_vec(&root));
    }
}
