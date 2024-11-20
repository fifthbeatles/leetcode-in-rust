/**
 * [295. Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/description/)
 */
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

/**
 * 比较通用的双Heap解法
 */
pub struct MedianFinderV1 {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
impl MedianFinderV1 {
    fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        if let Some(&Reverse(large_top)) = self.large.peek() {
            if let Some(&small_top) = self.small.peek() {
                if small_top > large_top {
                    self.large.push(Reverse(self.small.pop().unwrap()));
                }
            }
        }
        if self.small.len() > self.large.len() + 1 {
            self.large.push(Reverse(self.small.pop().unwrap()));
        } else if self.large.len() > self.small.len() + 1 {
            self.small.push(self.large.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        match self.small.len().cmp(&self.large.len()) {
            Ordering::Equal => {
                (*self.small.peek().unwrap() as f64 + self.large.peek().unwrap().0 as f64) / 2.0
            }
            Ordering::Greater => *self.small.peek().unwrap() as f64,
            Ordering::Less => self.large.peek().unwrap().0 as f64,
        }
    }
}

#[derive(Debug, Default)]
pub struct Median {
    middle: i32, // 下标为(size-1)/2的值，奇数个时就是中位数，偶数个时是中间两个左边的那个
    less_than_middle: usize, // 比middle小的数的个数
}

#[allow(dead_code)]
impl Median {
    fn new(middle: i32, less_than_middle: usize) -> Self {
        Self {
            middle,
            less_than_middle,
        }
    }
}

/**
 * 额外想到的一种解法
 */
#[derive(Debug, Default)]
pub struct MedianFinderV2 {
    pub counts: BTreeMap<i32, usize>,
    pub size: usize,
    pub median: Option<Median>,
}

#[allow(dead_code)]
impl MedianFinderV2 {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        if let Some(median) = self.median.as_mut() {
            *self.counts.entry(num).or_insert(0) += 1;
            self.size += 1;
            if num < median.middle {
                median.less_than_middle += 1;
            }
        } else {
            // 针对第一次特别处理，初始化middle
            self.size = 1;
            self.median = Some(Median::new(num, 0));
            self.counts.insert(num, 1);
        }
    }

    fn find_median(&mut self) -> f64 {
        if let Some(median) = self.median.as_mut() {
            let &old_middle_count = self.counts.get(&median.middle).unwrap();
            let middle_pos = (self.size - 1) / 2;
            if median.less_than_middle > middle_pos {
                // 左移了
                let move_left = median.less_than_middle - middle_pos;
                let mut c = 0;
                for (&num, &count) in self.counts.range(..median.middle).rev() {
                    c += count;
                    if c >= move_left {
                        median.middle = num;
                        median.less_than_middle -= c;
                        break;
                    }
                    continue;
                }
            } else if median.less_than_middle + old_middle_count <= middle_pos {
                // 右移了
                let move_right = middle_pos - (median.less_than_middle + old_middle_count) + 1;
                let mut c = 0;
                for (&num, &count) in self.counts.range(median.middle + 1..) {
                    if c + count >= move_right {
                        median.middle = num;
                        median.less_than_middle += old_middle_count + c;
                        break;
                    }
                    c += count;
                    continue;
                }
            } else {
                // 没变
            }
            if self.size % 2 == 1 {
                median.middle as f64
            } else {
                let &middle_count = self.counts.get(&median.middle).unwrap();
                if median.less_than_middle + middle_count >= middle_pos + 2 {
                    median.middle as f64
                } else {
                    let mut it = self.counts.range(median.middle + 1..);
                    (median.middle + *it.next().unwrap().0) as f64 / 2f64
                }
            }
        } else {
            // 虽然题目里会限制不会在为空时调用，但还是处理一下
            0f64
        }
    }
}

pub type MedianFinder = MedianFinderV1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        assert_eq!(median_finder.find_median().to_string(), "1".to_string());
        median_finder.add_num(2);
        assert_eq!(median_finder.find_median().to_string(), "1.5".to_string());
        median_finder.add_num(3);
        assert_eq!(median_finder.find_median().to_string(), "2".to_string());
    }

    #[test]
    fn test_case2() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(0);
        assert_eq!(median_finder.find_median().to_string(), "0".to_string());
        median_finder.add_num(0);
        assert_eq!(median_finder.find_median().to_string(), "0".to_string());
    }

    #[test]
    fn test_case3() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(6);
        assert_eq!(median_finder.find_median().to_string(), "6".to_string());
        median_finder.add_num(10);
        assert_eq!(median_finder.find_median().to_string(), "8".to_string());
        median_finder.add_num(2);
        assert_eq!(median_finder.find_median().to_string(), "6".to_string());
        median_finder.add_num(6);
        assert_eq!(median_finder.find_median().to_string(), "6".to_string());
        median_finder.add_num(5);
        assert_eq!(median_finder.find_median().to_string(), "6".to_string());
        median_finder.add_num(0);
        assert_eq!(median_finder.find_median().to_string(), "5.5".to_string());
        median_finder.add_num(6);
        assert_eq!(median_finder.find_median().to_string(), "6".to_string());
        median_finder.add_num(3);
        assert_eq!(median_finder.find_median().to_string(), "5.5".to_string());
        median_finder.add_num(1);
        assert_eq!(median_finder.find_median().to_string(), "5".to_string());
        median_finder.add_num(0);
        assert_eq!(median_finder.find_median().to_string(), "4".to_string());
        median_finder.add_num(0);
        assert_eq!(median_finder.find_median().to_string(), "3".to_string());
    }
}
