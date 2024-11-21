/**
 * [23. Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/description/)
 */
pub struct Solution;

use crate::data_structures::singly_linked_list::ListNode;

use std::collections::VecDeque;
use std::mem;

#[allow(dead_code)]
impl Solution {
    /**
     * 两两合并，尽量保证每次比较的两个队列长度接近
     */
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue: VecDeque<_> = lists.iter().cloned().collect();
        loop {
            if let Some(list1) = queue.pop_front() {
                if let Some(list2) = queue.pop_front() {
                    let new_list = Solution::merge_two_lists(list1, list2);
                    queue.push_back(new_list);
                } else {
                    return list1;
                }
            } else {
                return None;
            }
        }
    }

    /**
     * 参考了别人写的合并两个链表的方法
     */
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut ptr = &mut head;

        while list1.is_some() && list2.is_some() {
            let l1 = &mut list1;
            let l2 = &mut list2;
            let tmp = if l1.as_ref().unwrap().val > l2.as_ref().unwrap().val {
                l2
            } else {
                l1
            };
            mem::swap(tmp, ptr);
            mem::swap(tmp, &mut ptr.as_mut().unwrap().next);
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        mem::swap(
            ptr,
            if list1.is_none() {
                &mut list2
            } else {
                &mut list1
            },
        );
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]
            .iter()
            .map(|v| ListNode::from_vec(v))
            .collect();
        let new_list = Solution::merge_k_lists(lists);
        assert_eq!(
            new_list.unwrap().to_string(),
            "1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6 -> None"
        );
    }

    #[test]
    fn test_case2() {
        let lists = vec![];
        let new_list = Solution::merge_k_lists(lists);
        assert_eq!(new_list, None);
    }

    #[test]
    fn test_case3() {
        let lists = vec![vec![]].iter().map(|v| ListNode::from_vec(v)).collect();
        let new_list = Solution::merge_k_lists(lists);
        assert_eq!(new_list, None);
    }
}
