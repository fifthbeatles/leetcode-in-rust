/**
 * [278. First Bad Version](https://leetcode.com/problems/first-bad-version/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut high = n;
        let mut low = 1;
        while low < high {
            // 不直接写(high+low)/2，避免越界
            let mid = low + (high - low) / 2;
            if self.is_bad_version(mid) {
                high = mid;
            } else {
                low = mid + 1
            }
        }
        low
    }

    pub fn is_bad_version(&self, _version: i32) -> bool {
        true
    }
}
