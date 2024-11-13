/**
 * [155. Min Stack](https://leetcode.com/problems/min-stack/description/)
 */

#[derive(Default)]
pub struct MinStack {
    // 对于通用类型，第二个元素存储最小值所在的index可以减少存储空间，但这道题是i32，直接存储最小值会更快（减少一次地址转换）
    pub stack: Vec<(i32, usize)>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack::default()
    }

    fn push(&mut self, val: i32) {
        if let Some(pre_min) = self.stack.last() {
            if val < self.stack[pre_min.1].0 {
                self.stack.push((val, self.stack.len()));
            } else {
                self.stack.push((val, pre_min.1));
            }
        } else {
            self.stack.push((val, 0));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack[self.stack.last().unwrap().1].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn test_case2() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-1);
        assert_eq!(stack.get_min(), -2);
        assert_eq!(stack.top(), -1);
        stack.pop();
        assert_eq!(stack.get_min(), -2);
    }
}
