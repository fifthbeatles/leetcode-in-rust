/**
 * [232. Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/description/)
 */

struct MyQueue {
    data_stack: Vec<i32>,
    temp_stack: Vec<i32>,
    peek_value: Option<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            data_stack: vec![],
            temp_stack: vec![],
            peek_value: None,
        }
    }

    fn push(&mut self, x: i32) {
        if self.empty() {
            self.peek_value = Some(x);
        }
        self.data_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        while let Some(v) = self.data_stack.pop() {
            self.temp_stack.push(v);
        }
        let ret = self.temp_stack.pop().unwrap();
        if self.temp_stack.is_empty() {
            self.peek_value = None
        } else {
            self.peek_value = Some(*self.temp_stack.last().unwrap());
        }
        while let Some(v) = self.temp_stack.pop() {
            self.data_stack.push(v);
        }
        ret
    }

    fn peek(&self) -> i32 {
        self.peek_value.unwrap()
    }

    fn empty(&self) -> bool {
        self.data_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1);
        my_queue.push(2);
        assert_eq!(my_queue.peek(), 1);
        assert_eq!(my_queue.pop(), 1);
        assert_eq!(my_queue.empty(), false);
    }
}
