use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(values: &Vec<i32>) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }
        Some(Box::new(values.into()))
    }
}

impl From<&Vec<i32>> for ListNode {
    fn from(values: &Vec<i32>) -> Self {
        assert!(!values.is_empty());
        let mut iter = values.iter().rev();
        let value = iter.next().unwrap();
        let mut head = ListNode {
            val: *value,
            next: None,
        };
        for value in iter {
            head = ListNode {
                val: *value,
                next: Some(Box::new(head)),
            }
        }
        head
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        let mut cur = &self.next;
        while let Some(node) = cur {
            write!(f, " -> {}", node.val)?;
            cur = &node.next;
        }
        write!(f, " -> None")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let v: Vec<i32> = Vec::new();
        let head = ListNode::from_vec(&v);
        assert_eq!(head, None);
        let v = vec![1, 2, 3, 4, 5];
        let node = ListNode::from_vec(&v);
        println!("{v:?}");
        assert_eq!(node.unwrap().to_string(), "1 -> 2 -> 3 -> 4 -> 5 -> None");
    }
}
