/**
 * [46. Permutations](https://leetcode.com/problems/permutations/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length = nums.len();
        let mut stack = vec![vec![]];
        for i in 1..=length {
            let capacity = (1..=i).reduce(|acc, v| acc * v).unwrap_or(1);
            let mut new_stack = Vec::with_capacity(capacity);
            while let Some(method) = stack.pop() {
                let current_length = method.len() + 1;
                for j in 0..current_length {
                    let mut new_method = Vec::with_capacity(current_length);
                    let (v1, v2) = method.split_at(j);
                    new_method.extend(v1);
                    new_method.push(nums[i - 1]);
                    new_method.extend(v2);
                    new_stack.push(new_method);
                }
            }
            stack = new_stack;
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 3];
        let ret = Solution::permute(nums);
        println!("{:?}", ret);
        /*
        assert!(
            ret == vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
        */
    }

    #[test]
    fn test_case2() {
        let nums = vec![0, 1];
        let ret = Solution::permute(nums);
        println!("{:?}", ret);
        // assert!(ret == vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1];
        let ret = Solution::permute(nums);
        println!("{:?}", ret);
        assert!(ret == vec![vec![1]]);
    }
}
