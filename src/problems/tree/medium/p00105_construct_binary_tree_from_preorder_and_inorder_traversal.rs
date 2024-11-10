/**
 * [105. Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_recursive(&preorder, &inorder)
    }

    fn build_tree_recursive(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = preorder.len();
        if len != inorder.len() {
            return None;
        }
        preorder.first().and_then(|value| {
            inorder.iter().position(|v| *v == *value).map(|pos| {
                let left = Self::build_tree_recursive(&preorder[1..pos + 1], &inorder[0..pos]);
                let right =
                    Self::build_tree_recursive(&preorder[pos + 1..len], &inorder[pos + 1..len]);
                let node = TreeNode {
                    val: *value,
                    left,
                    right,
                };
                Rc::new(RefCell::new(node))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::to_vec;

    #[test]
    fn test_case1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = Solution::build_tree(preorder, inorder);
        assert_eq!(
            to_vec(&root),
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
        );
    }

    #[test]
    fn test_case2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let root = Solution::build_tree(preorder, inorder);
        assert_eq!(to_vec(&root), vec![Some(-1)]);
    }

    #[test]
    fn test_case3() {
        let preorder = vec![];
        let inorder = vec![];
        let root = Solution::build_tree(preorder, inorder);
        assert!(root.is_none());
    }
}
