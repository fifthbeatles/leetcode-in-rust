/**
 * [310. Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::find_min_height_trees_v1(n, edges)
    }

    pub fn find_min_height_trees_v1(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        if n == 2 {
            return vec![0, 1];
        }
        let mut degrees = vec![0; n];
        let mut neighbors = vec![vec![]; n];
        for edge in edges {
            let (n1, n2) = (edge[0] as usize, edge[1] as usize);
            degrees[n1] += 1;
            degrees[n2] += 1;
            neighbors[n1].push(n2);
            neighbors[n2].push(n1);
        }
        let mut leaf_nodes = degrees
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| if *v == 1 { Some(idx) } else { None })
            .collect::<Vec<_>>();
        loop {
            let mut new_leaf_nodes = vec![];
            for &node in &leaf_nodes {
                for &neighbor in &neighbors[node] {
                    degrees[neighbor] -= 1;
                    if degrees[neighbor] == 1 {
                        new_leaf_nodes.push(neighbor);
                    }
                }
            }
            if new_leaf_nodes.is_empty() {
                return vec![leaf_nodes[0] as i32, leaf_nodes[1] as i32];
            }
            if new_leaf_nodes.len() == 1 {
                return vec![new_leaf_nodes[0] as i32];
            }
            leaf_nodes = new_leaf_nodes;
        }
    }

    /**
     * 从LeetCode学到的优化方式，充分利用位异或的原理处理模拟集合操作
     */
    pub fn find_min_height_trees_v2(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        if n == 2 {
            return vec![0, 1];
        }
        let mut degrees = vec![0; n];
        let mut neighbors = vec![0; n];
        for edge in edges {
            let (n1, n2) = (edge[0] as usize, edge[1] as usize);
            degrees[n1] += 1;
            degrees[n2] += 1;
            neighbors[n1] ^= n2;
            neighbors[n2] ^= n1;
        }
        let mut leaf_nodes = degrees
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| if *v == 1 { Some(idx) } else { None })
            .collect::<Vec<_>>();
        loop {
            let mut new_leaf_nodes = vec![];
            for &node in &leaf_nodes {
                let neighbor = neighbors[node];
                // 因为只处理邻边为1的节点，所以对同一个节点进行再次异或计算就消掉了，只保留了最后一个临边的序号
                neighbors[neighbor] ^= node;
                degrees[neighbor] -= 1;
                if degrees[neighbor] == 1 {
                    new_leaf_nodes.push(neighbor);
                }
            }
            if new_leaf_nodes.is_empty() {
                return vec![leaf_nodes[0] as i32, leaf_nodes[1] as i32];
            }
            if new_leaf_nodes.len() == 1 {
                return vec![new_leaf_nodes[0] as i32];
            }
            leaf_nodes = new_leaf_nodes;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(Solution::find_min_height_trees(4, edges), vec![1]);
    }

    #[test]
    fn test_case2() {
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        assert_eq!(Solution::find_min_height_trees(6, edges), vec![3, 4]);
    }

    #[test]
    fn test_case3() {
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::find_min_height_trees(2, edges), vec![0, 1]);
    }
}
