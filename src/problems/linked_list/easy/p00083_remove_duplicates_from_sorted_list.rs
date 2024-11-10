/**
 * [83. Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/)
 */
pub struct Solution;

use crate::data_structures::singly_linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut head| {
            let mut prev_val = head.val;
            let mut new_head = ListNode {
                val: head.val,
                next: None,
            };
            let mut cur = &mut new_head;
            while let Some(node) = head.next.take() {
                if node.val != prev_val {
                    prev_val = node.val;
                    cur.next = Some(Box::new(ListNode {
                        val: node.val,
                        next: None,
                    }));
                    cur = cur.next.as_mut().unwrap();
                }
                head = node;
            }
            Box::new(new_head)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let list1 = ListNode::from_vec(&vec![1, 1, 2]);
        let list2 = Solution::delete_duplicates(list1);
        assert_eq!(list2.unwrap().to_string(), "1 -> 2 -> None");
    }

    #[test]
    fn test_case2() {
        let list1 = ListNode::from_vec(&vec![1, 1, 2, 3, 3]);
        let list2 = Solution::delete_duplicates(list1);
        assert_eq!(list2.unwrap().to_string(), "1 -> 2 -> 3 -> None");
    }
}
