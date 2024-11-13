/**
 * [207. Course Schedule](https://leetcode.com/problems/course-schedule/description/)
 */
pub struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        Solution::can_finish_v1(num_courses, prerequisites)
    }

    /**
     * BFS拓扑排序（Kahn算法）判断是否是有向无环图
     * 1、计算各节点的入度
     * 2、以所有入度为0的节点初始化队列
     * 3、从队首弹出元素加入拓扑排序结果
     * 4、对每个新被弹出的节点，其所有邻接节点的入度减1，将将入度新变为0的邻接节点入队
     * 5、重复3、4直到队列为空。如果所有节点都被排序，则图中无环，否则有环
     */
    pub fn can_finish_v1(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut neighbors = vec![vec![]; num_courses];
        let mut in_degrees = vec![0; num_courses];
        for prerequisite in prerequisites {
            neighbors[prerequisite[0] as usize].push(prerequisite[1] as usize);
            in_degrees[prerequisite[1] as usize] += 1;
        }
        let mut sorted = Vec::new();
        let mut queue = in_degrees
            .iter()
            .enumerate()
            .filter_map(|(idx, count)| if *count == 0 { Some(idx) } else { None })
            .collect::<VecDeque<_>>();
        while let Some(node) = queue.pop_front() {
            sorted.push(node);
            for neighbor in neighbors[node].iter() {
                in_degrees[*neighbor] -= 1;
                if in_degrees[*neighbor] == 0 {
                    queue.push_back(*neighbor);
                }
            }
        }
        sorted.len() == num_courses
    }

    /**
     * DFS遍历判断是否是有向无环图
     */
    pub fn can_finish_v2(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut neighbors = vec![vec![]; num_courses];
        for prerequisite in prerequisites {
            neighbors[prerequisite[0] as usize].push(prerequisite[1] as usize);
        }
        let mut visited = vec![0; num_courses];
        let mut stack = Vec::new();
        for node in 0..num_courses {
            if visited[node] == 0 {
                stack.push((node, 0));
                visited[node] = 1; // 标记为正在访问
                while let Some((current, order)) = stack.pop() {
                    let current_neighbors = &neighbors[current];
                    if let Some(&neighbor) = current_neighbors.get(order) {
                        stack.push((current, order + 1)); // 继续处理当前节点的下一个邻居
                        if visited[neighbor] == 0 {
                            stack.push((neighbor, 0));
                            visited[current] = 1;
                        } else if visited[neighbor] == 1 {
                            return false;
                        }
                    } else {
                        visited[current] = 2; // 标记为已访问
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let prerequisites = vec![vec![1, 0]];
        assert_eq!(Solution::can_finish(2, prerequisites), true);
    }

    #[test]
    fn test_case2() {
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(Solution::can_finish(2, prerequisites), false);
    }
}
