/**
 * [876. Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/description/)
 */
pub struct Solution;

use crate::data_structures::singly_linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while let Some(node1) = slow {
            if let Some(node2) = fast {
                fast = &node2.next;
            } else {
                break;
            }
            if let Some(node2) = fast {
                fast = &node2.next;
            } else {
                break;
            }
            slow = &node1.next;
        }
        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let list1 = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let list2 = Solution::middle_node(list1);
        assert_eq!(list2.unwrap().to_string(), "3 -> 4 -> 5 -> None");
    }

    #[test]
    fn test_case2() {
        let list1 = ListNode::from_vec(&vec![1, 2, 3, 4, 5, 6]);
        let list2 = Solution::middle_node(list1);
        assert_eq!(list2.unwrap().to_string(), "4 -> 5 -> 6 -> None");
    }
}
