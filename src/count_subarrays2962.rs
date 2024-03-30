use std::{i64, usize};

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let max: i32 = *nums
        .iter()
        .max()
        .expect("The nums vector was expected to be non-empty");
    let idxs = nums
        .iter()
        .enumerate()
        .filter(|(_, el)| **el == max)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    //there are not enough max elements
    if idxs.len() < k {
        0
    } else {
        let mut left = 0;
        let mut right = k - 1;
        let mut res: i64 = 0;
        while right < idxs.len() {
            let left_gap = if left == 0 {
                idxs[left]
            } else {
                idxs[left] - idxs[left - 1] - 1
            };
            let right_gap = nums.len() - 1 - idxs[right];
            let num_subs = (left_gap as i64 + 1) * (right_gap as i64 + 1);
            res += num_subs; //shift the interval
            left += 1;
            right += 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::count_subarrays2962::count_subarrays;

    #[test]
    fn zero_subarrays() {
        assert_eq!(count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }

    #[test]
    fn some_subarrays() {
        assert_eq!(count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
    }

    #[test]
    fn single_subarray() {
        assert_eq!(count_subarrays(vec![3, 3, 3, 3, 3], 5), 1)
    }

    #[test]
    fn lots_subarrays1() {
        assert_eq!(count_subarrays(vec![3, 3, 3], 1), 6)
    }

    #[test]
    fn lots_subarrays2() {
        assert_eq!(count_subarrays(vec![3, 3, 3], 2), 3)
    }
}
