/**
 * [15. 3Sum](https://leetcode.com/problems/3sum/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let counts = nums.iter().fold(HashMap::new(), |mut map, &item| {
            *map.entry(item).or_insert(0) += 1;
            map
        });
        // 三个元素一样，只能是0了
        if let Some(&count) = counts.get(&0) {
            if count >= 3 {
                ret.push(vec![0, 0, 0]);
            }
        }
        // 只有两个元素一样，即(x, x, -2x)
        for (&item, &count) in counts.iter() {
            if count >= 2 && item != 0 {
                let third_item = item * -2;
                if counts.contains_key(&third_item) {
                    if item > 0 {
                        ret.push(vec![third_item, item, item]);
                    } else {
                        ret.push(vec![item, item, third_item]);
                    }
                }
            }
        }
        // 三个元素都不相同
        let mut nums = counts.iter().map(|(&item, _)| item).collect::<Vec<_>>();
        nums.sort();
        let size = nums.len();
        for i in 0..size {
            for j in i + 1..size {
                let k = -nums[i] - nums[j];
                if k > nums[j] && counts.contains_key(&k) {
                    ret.push(vec![nums[i], nums[j], k]);
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ret = Solution::three_sum(nums);
        assert!(
            ret == vec![vec![-1, -1, 2], vec![-1, 0, 1]]
                || ret == vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        );
    }

    #[test]
    fn test_case2() {
        let nums = vec![0, 1, 1];
        let ret = Solution::three_sum(nums);
        assert!(ret.is_empty());
    }

    #[test]
    fn test_case3() {
        let nums = vec![0, 0, 0];
        let ret = Solution::three_sum(nums);
        assert_eq!(ret, vec![vec![0, 0, 0]]);
    }
}
