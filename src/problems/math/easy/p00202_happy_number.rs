/**
 * [202. Happy Number](https://leetcode.com/problems/happy-number/description/)
 */
pub struct Solution;

use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut nums: HashSet<i32> = HashSet::new();
        while n != 1 && !nums.contains(&n) {
            nums.insert(n);
            let mut m = n;
            n = 0;
            while m > 0 {
                n += (m % 10) * (m % 10);
                m /= 10;
            }
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
