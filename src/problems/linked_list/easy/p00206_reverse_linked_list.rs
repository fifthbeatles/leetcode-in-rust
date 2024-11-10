/**
 * [206. Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/description/)
 */
pub struct Solution;

use crate::data_structures::singly_linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_list_v2(head)
    }

    pub fn reverse_list_v1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(node1) = head {
            let mut new_head = ListNode {
                val: node1.val,
                next: None,
            };
            let mut cur = &node1.next;
            while let Some(node2) = cur {
                new_head = ListNode {
                    val: node2.val,
                    next: Some(Box::new(new_head)),
                };
                cur = &node2.next
            }
            Some(Box::new(new_head))
        } else {
            None
        }
    }

    fn reverse_list_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take(); // 取出下一个节点
            node.next = prev; // 反转指针
            prev = Some(node); // 移动 prev 到当前节点
        }

        prev // 返回反转后的头节点
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let list1 = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let list2 = Solution::reverse_list(list1);
        assert_eq!(list2.unwrap().to_string(), "5 -> 4 -> 3 -> 2 -> 1 -> None");
    }

    #[test]
    fn test_case2() {
        let list1 = ListNode::from_vec(&vec![1, 2]);
        let list2 = Solution::reverse_list(list1);
        assert_eq!(list2.unwrap().to_string(), "2 -> 1 -> None");
    }

    #[test]
    fn test_case3() {
        let list1 = ListNode::from_vec(&vec![1]);
        let list2 = Solution::reverse_list(list1);
        assert_eq!(list2.unwrap().to_string(), "1 -> None");
    }

    #[test]
    fn test_case4() {
        let list1 = ListNode::from_vec(&vec![]);
        let list2 = Solution::reverse_list(list1);
        assert_eq!(list2, None);
    }
}
