/**
 * [110. Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::postorder_non_recursive_fn;
use crate::data_structures::binary_tree::postorder_recursive_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_v1(root)
    }

    pub fn is_balanced_v1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let ret = postorder_non_recursive_fn::<(bool, i32)>(
            &root,
            &mut |_| (true, 1),
            &mut || (true, 0),
            &mut |left, right, _| {
                let balanced = (left.1 - right.1).abs() <= 1;
                (
                    !balanced, // 发现不平衡立即结束，所以也不需要像递归一样考虑子节点是否平衡
                    (balanced, (left.1.max(right.1) + 1)), // (是否平衡, 高度)
                )
            },
        );
        ret.0
    }

    pub fn is_balanced_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let ret = postorder_recursive_fn::<(bool, i32)>(
            &root,
            &mut |left, right, _| {
                (
                    left.0 && right.0 && (left.1 - right.1).abs() <= 1, // 是否平衡
                    left.1.max(right.1) + 1,                            // 高度
                )
            },
            &mut || (true, 0),
        );
        ret.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::{from_vec_str, to_vec};

    #[test]
    fn test_case1() {
        let root = from_vec_str("3,9,20,null,null,15,7");
        assert_eq!(Solution::is_balanced_v1(root.clone()), true);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::is_balanced_v2(root.clone()), true);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("1,2,2,3,3,null,null,4,4");
        assert_eq!(Solution::is_balanced(root.clone()), false);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        assert_eq!(Solution::is_balanced(root.clone()), true);
        println!("{:?}", to_vec(&root));
    }
}
