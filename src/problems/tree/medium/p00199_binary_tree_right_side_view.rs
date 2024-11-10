/**
 * [199. Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::level_order_traversal_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        level_order_traversal_fn(&root, |node, level| {
            if let Some(v) = ret.get_mut(level as usize) {
                *v = node.borrow().val;
            } else {
                ret.push(node.borrow().val);
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
        let root = from_vec_str("1,2,3,null,5,null,4");
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("1,null,3");
        assert_eq!(Solution::right_side_view(root), vec![1, 3]);
    }

    #[test]
    fn test_case3() {
        let root = from_vec_str("");
        assert_eq!(Solution::right_side_view(root), vec![]);
    }
}
