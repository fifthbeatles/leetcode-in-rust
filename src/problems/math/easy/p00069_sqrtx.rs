/**
 * [69. Sqrt(x)](https://leetcode.com/problems/sqrtx/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        Self::my_sqrt_v1(x)
    }

    pub fn my_sqrt_v1(x: i32) -> i32 {
        if x <= 0 {
            return 0;
        }
        for n in 1..=46340 {
            if n * n > x {
                return n - 1;
            }
        }
        46340
    }

    /**
     * Newton-Raphson algorithm
     */
    pub fn my_sqrt_v2(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut x = n / 2;
        loop {
            let y = (x + n / x) / 2;
            if y >= x {
                return x;
            }
            x = y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt_v2(4), 2);
        assert_eq!(Solution::my_sqrt_v2(8), 2);
    }
}
