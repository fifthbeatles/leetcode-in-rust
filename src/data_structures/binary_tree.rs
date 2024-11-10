use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
 * 从数组读取，None表示空叶子节点
 */
#[allow(dead_code)]
pub fn from_vec(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue = VecDeque::new();
    let mut iter = values.iter();
    let root = if let Some(&Some(v)) = iter.next() {
        let node = Rc::new(RefCell::new(TreeNode::new(v)));
        queue.push_back(node.clone());
        Some(node)
    } else {
        None
    };
    while let Some(parent_node) = queue.pop_front() {
        if let Some(v1) = iter.next() {
            if let Some(v2) = v1 {
                let node = Rc::new(RefCell::new(TreeNode::new(*v2)));
                queue.push_back(node.clone());
                parent_node.borrow_mut().left = Some(node);
            }
        } else {
            break;
        }
        if let Some(v1) = iter.next() {
            if let Some(v2) = v1 {
                let node = Rc::new(RefCell::new(TreeNode::new(*v2)));
                queue.push_back(node.clone());
                parent_node.borrow_mut().right = Some(node);
            }
        } else {
            break;
        }
    }
    root
}

/**
 * 从数组形式的字符串中读取以处理LeetCode示例的输入
 * 形如：1,2,3,4,5,null,8,null,null,6,7,9（不包括[]）
 */
#[allow(dead_code)]
pub fn from_vec_str(value: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let vec: Vec<Option<i32>> = value.split(",").map(|v| v.parse::<i32>().ok()).collect();
    from_vec(&vec)
}

/**
 * 类似层次遍历，但包含必要的None表明树的结构，与from_vec（当输入数据合法时）互逆
 */
#[allow(dead_code)]
pub fn to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    if root.is_none() {
        return Vec::new();
    }
    let mut queue = VecDeque::new();
    let mut ret = Vec::new();
    queue.push_back(root.clone());
    let mut cached_none_count = 0;
    while let Some(node1) = queue.pop_front() {
        if let Some(node2) = node1 {
            for _ in 0..cached_none_count {
                ret.push(None);
            }
            cached_none_count = 0;
            ret.push(Some(node2.borrow().val));
            queue.push_back(node2.borrow().left.clone());
            queue.push_back(node2.borrow().right.clone());
        } else {
            cached_none_count += 1;
        }
    }
    ret
}

/**
 * 层次遍历
 */
#[allow(dead_code)]
pub fn level_order_traversal_fn(
    root: &Option<Rc<RefCell<TreeNode>>>,
    mut func: impl FnMut(Rc<RefCell<TreeNode>>, u32) -> bool,
) {
    if root.is_none() {
        return;
    }
    let mut queue = VecDeque::new();
    queue.push_back((root.clone(), 0u32));
    while let Some((node1, level)) = queue.pop_front() {
        if let Some(node2) = node1 {
            let return_at_once = func(node2.clone(), level);
            if return_at_once {
                break;
            }
            queue.push_back((node2.borrow().left.clone(), level + 1));
            queue.push_back((node2.borrow().right.clone(), level + 1));
        }
    }
}

/**
 * 层次遍历
 */
#[allow(dead_code)]
pub fn level_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    level_order_traversal_fn(root, |node, _| {
        ret.push(node.borrow().val);
        false
    });
    ret
}

/**
 * 前序遍历，非递归
 */
#[allow(dead_code)]
pub fn preorder_traversal_fn(
    root: &Option<Rc<RefCell<TreeNode>>>,
    mut func: impl FnMut(Rc<RefCell<TreeNode>>) -> bool,
) {
    if root.is_none() {
        return;
    }
    let mut stack = Vec::new();
    stack.push(root.clone());
    while let Some(node1) = stack.pop() {
        if let Some(node2) = node1 {
            let return_at_once = func(node2.clone());
            if return_at_once {
                break;
            }
            stack.push(node2.borrow_mut().right.clone());
            stack.push(node2.borrow_mut().left.clone());
        }
    }
}

/**
 * 前序遍历，非递归
 */
#[allow(dead_code)]
pub fn preorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    preorder_traversal_fn(root, |node| {
        ret.push(node.borrow().val);
        false
    });
    ret
}

/**
 * 中序遍历，非递归
 */
#[allow(dead_code)]
pub fn inorder_traversal_fn(
    root: &Option<Rc<RefCell<TreeNode>>>,
    mut func: impl FnMut(Rc<RefCell<TreeNode>>) -> bool,
) {
    let mut stack = Vec::new();
    stack.push((false, root.clone()));
    while let Some((visited, Some(node))) = stack.pop() {
        if !visited {
            stack.push((true, Some(node.clone())));
            if let Some(left) = node.borrow().left.clone() {
                stack.push((false, Some(left.clone())));
            }
        } else {
            let return_at_once = func(node.clone());
            if return_at_once {
                break;
            }
            if let Some(right) = node.borrow().right.clone() {
                stack.push((false, Some(right.clone())));
            }
        }
    }
}

/**
 * 中序遍历，非递归
 */
#[allow(dead_code)]
pub fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    inorder_traversal_fn(root, |node| {
        ret.push(node.borrow().val);
        false
    });
    ret
}

/**
 * 后序遍历，非递归
 */
#[allow(dead_code)]
pub fn postorder_traversal_fn(
    root: &Option<Rc<RefCell<TreeNode>>>,
    mut func: impl FnMut(Rc<RefCell<TreeNode>>) -> bool,
) {
    let mut stack = Vec::new();
    stack.push((false, root.clone()));
    while let Some((visited, Some(node))) = stack.pop() {
        if !visited {
            stack.push((true, Some(node.clone())));
            if let Some(child) = node.borrow().right.clone() {
                stack.push((false, Some(child.clone())));
            }
            if let Some(child) = node.borrow().left.clone() {
                stack.push((false, Some(child.clone())));
            }
        } else {
            let return_at_once = func(node.clone());
            if return_at_once {
                break;
            }
        }
    }
}

/**
 * 后序遍历，非递归
 */
#[allow(dead_code)]
pub fn postorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    postorder_traversal_fn(root, |node| {
        ret.push(node.borrow().val);
        false
    });
    ret
}

/**
 * 递归后序遍历，主要用于根节点处理需要依赖子孙节点的场景
 */
#[allow(dead_code)]
pub fn postorder_recursive_fn<T: Copy>(
    root: &Option<Rc<RefCell<TreeNode>>>,
    func: &mut impl FnMut(T, T, i32) -> T, // 传入左/右子节点的处理结果和当前节点的值，返回当前节点的处理结果
    default_func: &mut impl FnMut() -> T,
) -> T {
    if let Some(node) = root.clone() {
        let left = postorder_recursive_fn(&node.borrow().left, func, default_func);
        let right = postorder_recursive_fn(&node.borrow().right, func, default_func);
        func(left, right, node.borrow().val)
    } else {
        default_func()
    }
}

/**
 * 非递归后序遍历，主要用于根节点处理需要依赖子孙节点的场景
 */
#[allow(dead_code)]
pub fn postorder_non_recursive_fn<T: Copy>(
    root: &Option<Rc<RefCell<TreeNode>>>,
    func1: &mut impl FnMut(Rc<RefCell<TreeNode>>) -> T,
    default_func: &mut impl FnMut() -> T,
    func2: &mut impl FnMut(T, T, T) -> (bool, T), // 传入左/右子节点的值和当前节点的原始值，计算当前节点的新值，并判断是否提前中止
) -> T {
    let mut queue = VecDeque::new();
    let mut parent_pos_vec: Vec<(Option<usize>, T)> = Vec::new();
    queue.push_back((root.clone(), None));
    while let Some((node1, parent_pos)) = queue.pop_front() {
        let new_pos_value = parent_pos_vec.len();
        if let Some(node2) = node1 {
            parent_pos_vec.push((parent_pos, func1(node2.clone())));
            queue.push_back((node2.borrow().left.clone(), Some(new_pos_value)));
            queue.push_back((node2.borrow().right.clone(), Some(new_pos_value)));
        } else {
            // 所有的空节点也保留，这样除根节点外其他节点都成对且相邻出现，总节点数一定是2N+1
            parent_pos_vec.push((parent_pos, default_func()));
        }
    }
    for i in (0..parent_pos_vec.len() / 2).rev() {
        let parent_pos = parent_pos_vec[2 * i + 1].0.unwrap();
        let (return_at_once, new_parent_value) = func2(
            parent_pos_vec[2 * i + 1].1,
            parent_pos_vec[2 * i + 2].1,
            parent_pos_vec[parent_pos].1,
        );
        parent_pos_vec[parent_pos].1 = new_parent_value;
        if return_at_once {
            return parent_pos_vec[parent_pos].1;
        }
    }
    parent_pos_vec[0].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec_and_to_vec() {
        let v: Vec<Option<i32>> = Vec::new();
        let root = from_vec(&v);
        assert_eq!(root, None);

        let v: Vec<Option<i32>> = vec![None];
        let root = from_vec(&v);
        assert_eq!(root, None);

        let v1: Vec<Option<i32>> = vec![Some(1), None, None];
        let root = from_vec(&v1);
        let v2 = to_vec(&root);
        assert_eq!(v2, vec![Some(1)]);

        let v1: Vec<Option<i32>> = vec![Some(1), None, None, Some(2)];
        let root = from_vec(&v1);
        let v2 = to_vec(&root);
        assert_eq!(v2, vec![Some(1)]);

        let v1 = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(8),
            None,
            None,
            Some(6),
            Some(7),
            Some(9),
        ];
        let root = from_vec(&v1);
        let v2 = to_vec(&root);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_from_vec_str() {
        let root = from_vec_str("");
        assert_eq!(root, None);

        let root = from_vec_str("null");
        assert_eq!(root, None);

        let root = from_vec_str("1,null,null,2");
        let v = to_vec(&root);
        assert_eq!(v, vec![Some(1)]);

        let root = from_vec_str("1,2,3,4,5,null,8,null,null,6,7,9");
        let v1 = to_vec(&root);
        let v2: Vec<Option<i32>> = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(8),
            None,
            None,
            Some(6),
            Some(7),
            Some(9),
        ];
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_traversal_case1() {
        let root = from_vec_str("");
        assert_eq!(preorder_traversal(&root), Vec::new());
        assert_eq!(inorder_traversal(&root), Vec::new());
        assert_eq!(postorder_traversal(&root), Vec::new());
        assert_eq!(level_order_traversal(&root), Vec::new());
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_traversal_case2() {
        let root = from_vec_str("1");
        assert_eq!(preorder_traversal(&root), vec![1]);
        assert_eq!(inorder_traversal(&root), vec![1]);
        assert_eq!(postorder_traversal(&root), vec![1]);
        assert_eq!(level_order_traversal(&root), vec![1]);
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_traversal_case3() {
        let root = from_vec_str("1,2,3,4,5,null,8,null,null,6,7,9");
        assert_eq!(preorder_traversal(&root), vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
        assert_eq!(inorder_traversal(&root), vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
        assert_eq!(postorder_traversal(&root), vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
        assert_eq!(
            level_order_traversal(&root),
            vec![1, 2, 3, 4, 5, 8, 6, 7, 9]
        );
        println!("{:?}", to_vec(&root));
    }

    #[test]
    fn test_postorder_recursive_fn() {
        let root = from_vec_str("1,2,3,4,5,null,8,null,null,6,7,9");
        let mut vec: Vec<i32> = Vec::new();
        let mut func = |_: (), _: (), value| {
            vec.push(value);
        };
        let mut default_func = || ();
        postorder_recursive_fn(&root, &mut func, &mut default_func);
        assert_eq!(vec, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
        println!("{:?}", to_vec(&root));
    }
}
