/**
 * [67. Add Binary](https://leetcode.com/problems/add-binary/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut va: Vec<u8> = a
            .chars()
            .rev()
            .map(|x| match x {
                '1' => 1,
                _ => 0,
            })
            .collect();
        let mut vb: Vec<u8> = b
            .chars()
            .rev()
            .map(|x| match x {
                '1' => 1,
                _ => 0,
            })
            .collect();
        let mut vc = if va.len() <= vb.len() {
            for i in 0..va.len() {
                vb[i] += va[i]
            }
            vb
        } else {
            for i in 0..vb.len() {
                va[i] += vb[i]
            }
            va
        };
        let last_key = vc.len() - 1;
        for i in 0..last_key {
            if vc[i] >= 2 {
                vc[i] -= 2;
                vc[i + 1] += 1;
            }
        }
        if vc[last_key] >= 2 {
            vc[last_key] -= 2;
            vc.push(1);
        }
        vc.into_iter()
            .rev()
            .map(|i| if i == 1 { '1' } else { '0' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
