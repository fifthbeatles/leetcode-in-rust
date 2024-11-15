/**
 * [62. Unique Paths](https://leetcode.com/problems/unique-paths/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        Solution::unique_paths_v2(m, n)
    }

    pub fn unique_paths_v1(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![1; n]; m];
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m - 1][n - 1]
    }

    /**
     * 数学方法，其实就是对应了组合数C(m-1, m+n-2)或者C(n-1, m+n-2)
     */
    pub fn unique_paths_v2(m: i32, n: i32) -> i32 {
        let (m, n) = if m <= n { (m, n) } else { (n, m) };
        if m == 1 {
            return 1;
        }
        let mut ret = 1u128;
        for i in 0..m - 1 {
            ret *= (n + i) as u128;
        }
        for i in 1..=m - 1 {
            ret /= i as u128;
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
