/**
 * [1235. Maximum Profit in Job Scheduling](https://leetcode.com/problems/maximum-profit-in-job-scheduling/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        Solution::job_scheduling_v2(start_time, end_time, profit)
    }

    pub fn job_scheduling_v1(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((x, y), z)| (*x, *y, *z))
            .collect::<Vec<_>>();
        jobs.sort();
        let mut dp = vec![(jobs[0].2, jobs[0].2, jobs[0].1)];
        for idx1 in 1..jobs.len() {
            let job = jobs[idx1];
            let mut current_max = job.2;
            for idx2 in (0..idx1).rev() {
                let pre_job = jobs[idx2];
                let pre_dp = dp[idx2];
                // 和前面所有都没覆盖
                if job.0 >= pre_dp.2 {
                    current_max = current_max.max(pre_dp.1 + job.2);
                    break;
                }
                if job.0 >= pre_job.1 {
                    // 和前一个不覆盖，但和更早之前的覆盖了
                    current_max = current_max.max(pre_dp.0 + job.2);
                } else if job.0 == pre_job.0 && job.2 <= pre_job.2 {
                    // 比前一个更差了
                    break;
                } else {
                    current_max = current_max.max(pre_dp.0 - pre_job.2 + job.2);
                }
            }
            dp.push((
                current_max,
                current_max.max(dp[idx1 - 1].1),
                dp[idx1 - 1].2.max(job.1),
            ));
        }
        dp.last().unwrap().1
    }

    pub fn job_scheduling_v2(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((x, y), z)| (*x, *y, *z))
            .collect::<Vec<_>>();
        jobs.sort();
        let size: usize = jobs.len();

        let mut dp = vec![0; size + 1];
        for idx1 in (0..size).rev() {
            let idx2 = jobs
                .binary_search_by_key(&(jobs[idx1].1, jobs[idx1].1), |(start, end, ..)| {
                    (*start, *end)
                })
                .unwrap_or_else(|v| v);
            dp[idx1] = (jobs[idx1].2 + dp[idx2]).max(dp[idx1 + 1]);
        }
        //println!("{:?}", jobs);
        //println!("{:?}", dp);
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 120);
    }

    #[test]
    fn test_case2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 150);
    }

    #[test]
    fn test_case3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 6);
    }

    #[test]
    fn test_case4() {
        let start_time = vec![6, 15, 7, 11, 1, 3, 16, 2];
        let end_time = vec![19, 18, 19, 16, 10, 8, 19, 8];
        let profit = vec![2, 9, 1, 19, 5, 7, 3, 19];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 41);
    }

    #[test]
    fn test_case5() {
        let start_time = vec![1, 2, 2, 3];
        let end_time = vec![2, 5, 3, 4];
        let profit = vec![3, 4, 1, 2];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 7);
    }
}
