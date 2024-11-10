/**
 * [104. Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::level_order_traversal_fn;
use crate::data_structures::binary_tree::postorder_recursive_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth_v1(root)
    }

    pub fn max_depth_v1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ret = 0;
        level_order_traversal_fn(&root, |_, level| {
            ret = level;
            false
        });
        (ret as i32) + 1
    }

    pub fn max_depth_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        postorder_recursive_fn::<i32>(&root, &mut |left, right, _| left.max(right) + 1, &mut || 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::{from_vec_str, to_vec};

    #[test]
    fn test_case1() {
        let root = from_vec_str("3,9,20,null,null,15,7");
        assert_eq!(Solution::max_depth_v1(root.clone()), 3);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::max_depth_v2(root.clone()), 3);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("1,null,2");
        assert_eq!(Solution::max_depth_v1(root.clone()), 2);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::max_depth_v2(root.clone()), 2);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        assert_eq!(Solution::max_depth_v1(root.clone()), 0);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::max_depth_v2(root.clone()), 0);
        println!("{:?}", to_vec(&root));
    }
}
