use std::fmt::{Display, Formatter};
use std::{cell::RefCell, rc::Rc};

pub type NodeRef<T> = Rc<RefCell<ListNode<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode<T: Copy> {
    pub value: T,
    pub prev: Option<NodeRef<T>>,
    pub next: Option<NodeRef<T>>,
}

#[allow(dead_code)]
impl<T: Copy> ListNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            prev: None,
            next: None,
        }
    }
}

/**
 * 简化版双向链表，参考了LeetCode其他人提交的代码。
 * 在解决[146. LRU Cache](https://leetcode.com/problems/lru-cache/description/)时，需要把节点的引用保存到Map里，
 * 而`std::collections::LinkedList`内部的Node是不可见的
 */
#[derive(Debug)]
pub struct DoublyList<T: Copy> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
    length: usize,
}

#[allow(dead_code)]
impl<T: Copy> DoublyList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_head(&self) -> Option<NodeRef<T>> {
        self.head.clone()
    }

    pub fn get_tail(&self) -> Option<NodeRef<T>> {
        self.tail.clone()
    }

    fn push_front_node(&mut self, node: NodeRef<T>) {
        node.borrow_mut().prev = None;
        node.borrow_mut().next = self.get_head();
        if let Some(head_node) = self.get_head() {
            head_node.borrow_mut().prev = Some(node.clone());
        } else {
            self.tail.replace(node.clone());
        }
        self.head.replace(node.clone());
        self.length += 1;
    }

    fn push_back_node(&mut self, node: NodeRef<T>) {
        node.borrow_mut().prev = self.get_tail();
        node.borrow_mut().next = None;
        if let Some(tail_node) = self.get_tail() {
            tail_node.borrow_mut().next = Some(node.clone());
        } else {
            self.head.replace(node.clone());
        }
        self.tail.replace(node.clone());
        self.length += 1;
    }

    pub fn push_front(&mut self, value: T) {
        self.push_front_node(Rc::new(RefCell::new(ListNode::new(value))));
    }

    pub fn push_back(&mut self, value: T) {
        self.push_back_node(Rc::new(RefCell::new(ListNode::new(value))));
    }

    pub fn remove(&mut self, target: NodeRef<T>) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();

        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next.take();
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                next.borrow_mut().prev.take();
                self.head.replace(next);
            }
            (None, None) => {
                self.head.take();
                self.tail.take();
            }
        }
        self.length -= 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(node) = self.get_head() {
            let ret = Some(node.borrow().value);
            self.remove(node);
            return ret;
        }
        None
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(node) = self.get_tail() {
            let ret = Some(node.borrow().value);
            self.remove(node);
            return ret;
        }
        None
    }

    pub fn move_to_head(&mut self, target: NodeRef<T>) {
        if let Some(node) = self.get_head() {
            if !Rc::ptr_eq(&node, &target) {
                self.remove(target.clone());
                self.push_front_node(target);
            }
        }
    }

    pub fn move_to_tail(&mut self, target: NodeRef<T>) {
        if let Some(node) = self.get_tail() {
            if !Rc::ptr_eq(&node, &target) {
                self.remove(target.clone());
                self.push_back_node(target);
            }
        }
    }

    pub fn to_vec(&self, reverse_order: bool) -> Vec<T> {
        let mut v = vec![];
        if reverse_order {
            let mut cur = self.tail.clone();
            while let Some(node) = cur {
                v.push(node.borrow().value);
                cur = node.borrow().prev.clone();
            }
        } else {
            let mut cur = self.head.clone();
            while let Some(node) = cur {
                v.push(node.borrow().value);
                cur = node.borrow().next.clone();
            }
        }
        v
    }
}

impl<T: Copy> From<&Vec<T>> for DoublyList<T> {
    fn from(values: &Vec<T>) -> Self {
        let mut doubly_list = DoublyList::new();
        for value in values {
            doubly_list.push_back(*value);
        }
        doubly_list
    }
}

impl<T: Copy + Display> Display for DoublyList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut cur = self.head.clone();
        if let Some(head_node) = cur {
            write!(f, "{}", head_node.borrow().value)?;
            cur = head_node.borrow().next.clone();
            while let Some(node) = cur {
                write!(f, " <=> {}", node.borrow().value)?;
                cur = node.borrow().next.clone();
            }
            write!(f, "")
        } else {
            write!(f, "None")
        }
    }
}

impl<T: Copy> Default for DoublyList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_output() {
        let v: Vec<i32> = Vec::new();
        let doubly_list = DoublyList::from(&v);
        assert_eq!(doubly_list.get_head(), None);
        assert_eq!(doubly_list.to_string(), "None");

        let v = vec![1];
        let doubly_list = DoublyList::from(&v);
        assert_eq!(doubly_list.to_string(), "1");

        let v = vec![1, 2, 3, 4, 5];
        let doubly_list = DoublyList::from(&v);
        assert_eq!(doubly_list.to_string(), "1 <=> 2 <=> 3 <=> 4 <=> 5");
        let v1 = doubly_list.to_vec(false);
        let v2 = doubly_list.to_vec(true);
        assert_eq!(v1, vec![1, 2, 3, 4, 5]);
        assert_eq!(v2, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_push_pop() {
        let mut doubly_list = DoublyList::new();
        assert_eq!(doubly_list.pop_back(), None);
        assert_eq!(doubly_list.pop_front(), None);
        doubly_list.push_front(2);
        doubly_list.push_front(1);
        doubly_list.push_back(3);
        doubly_list.push_back(4);
        assert_eq!(doubly_list.len(), 4);
        let v1 = doubly_list.to_vec(false);
        let v2 = doubly_list.to_vec(true);
        assert_eq!(v1, vec![1, 2, 3, 4]);
        assert_eq!(v2, vec![4, 3, 2, 1]);
        assert_eq!(doubly_list.pop_front(), Some(1));
        assert_eq!(doubly_list.pop_back(), Some(4));
        assert_eq!(doubly_list.len(), 2);
        let v1 = doubly_list.to_vec(false);
        let v2 = doubly_list.to_vec(true);
        assert_eq!(v1, vec![2, 3]);
        assert_eq!(v2, vec![3, 2]);
        assert_eq!(doubly_list.pop_back(), Some(3));
        assert_eq!(doubly_list.pop_front(), Some(2));
        assert_eq!(doubly_list.len(), 0);
    }

    #[test]
    fn test_move() {
        let v = vec![1, 2, 3, 4, 5];
        let mut doubly_list = DoublyList::from(&v);
        doubly_list.move_to_head(doubly_list.get_tail().unwrap());
        let v1 = doubly_list.to_vec(false);
        let v2 = doubly_list.to_vec(true);
        assert_eq!(v1, vec![5, 1, 2, 3, 4]);
        assert_eq!(v2, vec![4, 3, 2, 1, 5]);
        doubly_list.move_to_tail(doubly_list.get_head().unwrap());
        let v1 = doubly_list.to_vec(false);
        let v2 = doubly_list.to_vec(true);
        assert_eq!(v1, vec![1, 2, 3, 4, 5]);
        assert_eq!(v2, vec![5, 4, 3, 2, 1]);
    }
}
