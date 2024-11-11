/**
 * [39. Combination Sum](https://leetcode.com/problems/combination-sum/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let candidates: Vec<usize> = candidates
            .iter()
            .filter_map(|&v| if v > target { None } else { Some(v as usize) })
            .collect();
        let target = target as usize;
        let mut methods = Vec::with_capacity(target + 1);
        methods.push(vec![Vec::new()]);
        for i in 1..=target {
            let mut current_methods = Vec::new();
            for &j in candidates.iter() {
                if i < j {
                    continue;
                }
                for method in methods[i - j].iter() {
                    if method.is_empty() || j as i32 >= method[method.len() - 1] {
                        let mut tmp: Vec<i32> = method.to_vec();
                        tmp.push(j as i32);
                        current_methods.push(tmp);
                    }
                }
            }
            methods.push(current_methods);
        }
        methods.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![2, 3, 6, 7];
        let ret = Solution::combination_sum(nums, 7);
        assert!(ret == vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_case2() {
        let nums = vec![2, 3, 5];
        let ret = Solution::combination_sum(nums, 8);
        assert!(ret == vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    #[test]
    fn test_case3() {
        let nums = vec![2];
        let ret = Solution::combination_sum(nums, 1);
        assert!(ret.is_empty());
    }
}
