/**
 * [230. Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::inorder_traversal_fn;
use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut counter = 0;
        let mut ret = i32::MIN;
        inorder_traversal_fn(&root, |node| {
            counter += 1;
            ret = node.borrow().val;
            counter >= k
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::{from_vec_str, to_vec};

    #[test]
    fn test_case1() {
        let root = from_vec_str("3,1,4,null,2");
        assert_eq!(Solution::kth_smallest(root.clone(), 1), 1);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_case2() {
        let root = from_vec_str("5,3,6,2,4,null,null,1");
        assert_eq!(Solution::kth_smallest(root.clone(), 3), 3);
        println!("{:?}", to_vec(&root));
    }
}
