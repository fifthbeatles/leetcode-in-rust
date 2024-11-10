/**
 * [235. Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/description/)
 */
pub struct Solution;

use crate::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lowest_common_ancestor_v1(root, p, q)
    }

    pub fn lowest_common_ancestor_v1(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p.is_none() || q.is_none() {
            return None;
        }
        let p_value = p.unwrap().borrow().val;
        let q_value = q.unwrap().borrow().val;
        let mut cur = root;
        while let Some(node) = cur {
            let cur_value = node.borrow().val;
            if p_value < cur_value && q_value < cur_value {
                cur = node.borrow().left.clone();
            } else if p_value > cur_value && q_value > cur_value {
                cur = node.borrow().right.clone();
            } else {
                return Some(node);
            }
        }
        None
    }

    pub fn lowest_common_ancestor_v2(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let path_p = Self::find_path_v2(&root, p);
        let path_q = Self::find_path_v2(&root, q);
        let mut ret = None;
        for i in 0..path_p.len().min(path_q.len()) {
            if path_p[i] == path_q[i] {
                ret = path_p[i].clone();
            } else {
                break;
            }
        }
        ret
    }

    /**
     * 忽略了题目是Binary Search Tree，仍不失为一种可行的方法
     */
    fn find_path_v1(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if target.is_none() {
            return Vec::new();
        }
        let target = target.clone().unwrap();
        let mut stack = Vec::new();
        let mut path = Vec::new();
        stack.push((false, root.clone()));
        while let Some((visited, Some(node))) = stack.pop() {
            if !visited {
                path.push(Some(node.clone()));
                if node == target {
                    break;
                }
                stack.push((true, Some(node.clone())));
                if let Some(child) = node.borrow().right.clone() {
                    stack.push((false, Some(child.clone())));
                }
                if let Some(child) = node.borrow().left.clone() {
                    stack.push((false, Some(child.clone())));
                }
            } else {
                path.pop();
            }
        }
        path
    }

    fn find_path_v2(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if target.is_none() {
            return Vec::new();
        }
        let target_value = target.clone().unwrap().borrow().val;
        let mut path = Vec::new();
        let mut cur = root.clone();
        while let Some(node) = cur.clone() {
            path.push(cur);
            match node.clone().borrow().val.cmp(&target_value) {
                Ordering::Equal => break,
                Ordering::Greater => cur = node.borrow().left.clone(),
                Ordering::Less => cur = node.borrow().right.clone(),
            }
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::binary_tree::{from_vec_str, level_order_traversal_fn};

    #[test]
    fn test_case1() {
        let root = from_vec_str("6,2,8,0,4,7,9,null,null,3,5");
        let mut node1 = None;
        let mut node2 = None;
        let mut node3 = None;
        let mut node4 = None;
        level_order_traversal_fn(&root, |node, _| {
            match node.borrow().val {
                2 => node1 = Some(node.clone()),
                8 => node2 = Some(node.clone()),
                4 => node3 = Some(node.clone()),
                6 => node4 = Some(node.clone()),
                _ => {}
            };
            false
        });
        let node5 = Solution::lowest_common_ancestor(root.clone(), node1.clone(), node2.clone());
        assert_eq!(node4, node5);
        let node6 = Solution::lowest_common_ancestor(root.clone(), node1.clone(), node3.clone());
        assert_eq!(node6, node1);
    }
}
