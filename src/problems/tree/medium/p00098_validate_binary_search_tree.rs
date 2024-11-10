/**
 * [98. Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::postorder_non_recursive_fn;
use crate::data_structures::binary_tree::postorder_recursive_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_v1(root)
    }

    pub fn is_valid_bst_v1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let ret = postorder_non_recursive_fn::<(bool, bool, i32, i32)>(
            &root,
            &mut |node| (true, false, node.borrow().val, node.borrow().val),
            &mut || (true, true, i32::MIN, i32::MAX),
            &mut |left, right, cur| {
                let valid = (left.1 || cur.2 > left.3) && (right.1 || cur.3 < right.2);
                (
                    !valid, // 发现不合法立即结束，所以也不需要像递归一样考虑子节点是否合法
                    (
                        valid,                                 // 是否合法
                        false,                                 // 当前节点是否是空节点
                        if left.1 { cur.2 } else { left.2 },   // 最小值
                        if right.1 { cur.3 } else { right.3 }, // 最大值
                    ),
                )
            },
        );
        ret.0
    }

    /**
     * 递归版本
     */
    pub fn is_valid_bst_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let ret = postorder_recursive_fn::<(bool, bool, i32, i32)>(
            &root,
            &mut |left, right, value| {
                let valid =
                    left.0 && (left.1 || value > left.3) && right.0 && (right.1 || value < right.2);
                (
                    valid,                                 // 是否合法
                    false,                                 // 当前节点是否是空节点
                    if left.1 { value } else { left.2 },   // 最小值
                    if right.1 { value } else { right.3 }, // 最大值
                )
            },
            &mut || (true, true, i32::MIN, i32::MAX),
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
        let root = from_vec_str("2,1,3");
        assert_eq!(Solution::is_valid_bst_v1(root.clone()), true);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::is_valid_bst_v2(root.clone()), true);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("5,1,4,null,null,3,6");
        assert_eq!(Solution::is_valid_bst_v1(root.clone()), false);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::is_valid_bst_v2(root.clone()), false);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        assert_eq!(Solution::is_valid_bst_v1(root.clone()), true);
        println!("{:?}", to_vec(&root));
        assert_eq!(Solution::is_valid_bst_v2(root.clone()), true);
        println!("{:?}", to_vec(&root));
    }
}
