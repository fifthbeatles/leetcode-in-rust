/**
 * [66. Plus One](https://leetcode.com/problems/plus-one/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        if digits.iter().all(|x| *x == 9) {
            let mut new_digits = vec![0; digits.len() + 1];
            new_digits[0] = 1;
            return new_digits;
        }
        let mut new_digits: Vec<i32> = digits.into_iter().collect();
        let mut add_one = true;
        for i in (0..new_digits.len()).rev() {
            if !add_one {
                break;
            }
            if new_digits[i] < 9 {
                new_digits[i] += 1;
                add_one = false;
            } else {
                new_digits[i] = 0;
                add_one = true;
            }
        }
        new_digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
