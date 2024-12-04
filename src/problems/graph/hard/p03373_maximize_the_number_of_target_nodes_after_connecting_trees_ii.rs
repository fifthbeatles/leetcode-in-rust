/**
 * [3373. Maximize the Number of Target Nodes After Connecting Trees II](https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/description/)
 */
pub struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let (even, even_count) = Solution::count_target(&edges2, 0);
        let common_count = even_count.max(even.len() - even_count) as i32;
        let (even, even_count) = Solution::count_target(&edges1, 0);
        let size = even.len();
        let odd_count = (size - even_count) as i32;
        let even_count = even_count as i32;
        let mut ret = Vec::with_capacity(size);
        for is_even in even {
            if is_even {
                ret.push(even_count + common_count);
            } else {
                ret.push(odd_count + common_count);
            }
        }
        ret
    }

    /**
     * 计算其他点与给定点的奇偶性关系
     */
    fn count_target(edges: &[Vec<i32>], init_node: usize) -> (Vec<bool>, usize) {
        let nodes = edges.len() + 1;
        let mut new_edges = vec![vec![]; nodes];
        for edge in edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            new_edges[n1].push(n2);
            new_edges[n2].push(n1);
        }
        let mut queue = VecDeque::new();
        let mut even = vec![false; nodes];
        even[init_node] = true;
        let mut odd = vec![false; nodes];
        queue.push_back((init_node, true));
        let mut even_count = 1usize;
        while let Some((node1, is_even)) = queue.pop_front() {
            if !is_even {
                for &node2 in &new_edges[node1] {
                    if !even[node2] {
                        queue.push_back((node2, true));
                        even[node2] = true;
                        even_count += 1;
                    }
                }
            } else {
                for &node2 in &new_edges[node1] {
                    if !odd[node2] {
                        queue.push_back((node2, false));
                        odd[node2] = true;
                    }
                }
            }
        }
        (even, even_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 7],
            vec![1, 4],
            vec![4, 5],
            vec![4, 6],
        ];
        assert_eq!(
            Solution::max_target_nodes(edges1, edges2),
            vec![8, 7, 7, 8, 8]
        );
    }

    #[test]
    fn test_case2() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(
            Solution::max_target_nodes(edges1, edges2),
            vec![3, 6, 6, 6, 6]
        );
    }
}
