/**
 * [621. Task Scheduler](https://leetcode.com/problems/task-scheduler/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let tasks_size = tasks.len() as i32;
        let mut counts = vec![0; 26];
        for &ch in &tasks {
            counts[(ch as u8 - b'A') as usize] += 1;
        }
        counts.sort_by(|a, b| b.cmp(a));
        while Some(&0) == counts.last() {
            counts.pop();
        }
        if counts[0] == 1 {
            return tasks_size;
        }
        let max_count = counts[0];
        let mut m = 1;
        while m < counts.len() && counts[m] == max_count {
            m += 1;
        }
        tasks_size.max((n + 1) * (max_count - 1) + m as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        assert_eq!(Solution::least_interval(tasks, 2), 8);
    }

    #[test]
    fn test_case2() {
        let tasks = vec!['A', 'C', 'A', 'B', 'D', 'B'];
        assert_eq!(Solution::least_interval(tasks, 1), 6);
    }

    #[test]
    fn test_case3() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        assert_eq!(Solution::least_interval(tasks, 3), 10);
    }
}
