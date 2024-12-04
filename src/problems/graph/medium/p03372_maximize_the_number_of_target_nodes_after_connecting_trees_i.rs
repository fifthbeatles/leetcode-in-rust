/**
 * [3372. Maximize the Number of Target Nodes After Connecting Trees I](https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/description/)
 */
pub struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let counts2 = Solution::count_target(&edges2, k - 1);
        let common_count = counts2.iter().max().unwrap();
        let mut counts1 = Solution::count_target(&edges1, k);
        for v in counts1.iter_mut() {
            *v += *common_count;
        }
        counts1
    }

    fn count_target(edges: &[Vec<i32>], target: i32) -> Vec<i32> {
        let nodes = edges.len() + 1;
        let mut new_edges = vec![vec![]; nodes];
        for edge in edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            new_edges[n1].push(n2);
            new_edges[n2].push(n1);
        }
        let mut counts = Vec::with_capacity(nodes);
        for node in 0..nodes {
            let mut distances = vec![i32::MAX; nodes];
            let mut queue = VecDeque::new();
            distances[node] = 0;
            queue.push_back((node, 0));
            while let Some((node1, d)) = queue.pop_front() {
                if d > distances[node1] {
                    continue;
                }
                let new_d = d + 1;
                if new_d > target {
                    break;
                }
                for &node2 in &new_edges[node1] {
                    if new_d < distances[node2] {
                        queue.push_back((node2, new_d));
                        distances[node2] = new_d;
                    }
                }
            }
            let c = (distances.iter().filter(|&&d| d <= target)).count();
            counts.push(c as i32);
        }
        counts
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
            Solution::max_target_nodes(edges1, edges2, 2),
            vec![9, 7, 9, 8, 8]
        );
    }

    #[test]
    fn test_case2() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(
            Solution::max_target_nodes(edges1, edges2, 1),
            vec![6, 3, 3, 3, 3]
        );
    }
}
