/**
 * [21. Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/description/)
 */
pub struct Solution;

use crate::data_structures::singly_linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }
        let mut tmp_head = ListNode {
            val: 0, //i32::default(),
            next: None,
        };
        let mut current = &mut tmp_head;
        while let (Some(mut node1), Some(mut node2)) = (list1.take(), list2.take()) {
            if node1.val <= node2.val {
                list2 = Some(node2);
                list1 = node1.next.take();
                current.next = Some(node1);
            } else {
                list1 = Some(node1);
                list2 = node2.next.take();
                current.next = Some(node2);
            }
            current = current.next.as_mut().unwrap();
            if list1.is_none() {
                current.next = list2;
                break;
            }
            if list2.is_none() {
                current.next = list1;
                break;
            }
        }
        tmp_head.next.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let list1 = ListNode::from_vec(&vec![1, 2, 4]);
        let list2 = ListNode::from_vec(&vec![1, 3, 4]);
        let list3 = Solution::merge_two_lists(list1, list2);
        assert_eq!(
            list3.unwrap().to_string(),
            "1 -> 1 -> 2 -> 3 -> 4 -> 4 -> None"
        );
    }

    #[test]
    fn test_case2() {
        let list1 = ListNode::from_vec(&Vec::new());
        let list2 = ListNode::from_vec(&Vec::new());
        let list3 = Solution::merge_two_lists(list1, list2);
        assert_eq!(list3, None);
    }

    #[test]
    fn test_case3() {
        let list1 = ListNode::from_vec(&Vec::new());
        let list2 = ListNode::from_vec(&vec![0]);
        let list3 = Solution::merge_two_lists(list1, list2);
        assert_eq!(list3.unwrap().to_string(), "0 -> None");
    }
}
