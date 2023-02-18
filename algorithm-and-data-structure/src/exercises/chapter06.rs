use std::cmp::Ordering;
use superslice::Ext;

pub fn nth_rank(nums: Vec<u32>) -> Vec<usize> {
    fn _binary_search(nums: Vec<u32>, target_num: u32, left: isize, right: isize) -> Option<usize> {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;

        match target_num.cmp(&nums[mid as usize]) {
            Ordering::Equal => Some(mid as usize),
            Ordering::Less => _binary_search(nums, target_num, left, mid - 1),
            Ordering::Greater => _binary_search(nums, target_num, mid + 1, right),
        }
    }

    let mut copied = nums.to_vec();
    let mut results = vec![0; nums.len()];
    copied.sort();
    for i in 0..nums.len() {
        let key = _binary_search(copied.to_vec(), nums[i], 0, nums.len() as isize - 1);
        results[i] = key.unwrap();
    }
    return results;
}

pub fn sum_of_three_sequences(mut a: Vec<u32>, mut b: Vec<u32>, mut c: Vec<u32>) -> u32 {
    a.sort();
    b.sort();
    c.sort();

    b.iter()
        .map(|b| a.lower_bound(&b) * (a.len() - c.upper_bound(&b)))
        .sum::<usize>() as u32
}

pub fn darts(mut a: Vec<u32>, limit: u32) -> u32 {
    a.push(0);
    let mut merged: Vec<u32> = vec![];

    for i in 0..a.len() {
        for j in 0..a.len() {
            merged.push(a[i] + a[j]);
        }
    }
    merged.sort();

    let mut result = 0;
    for m in 0..merged.len() {
        if limit < merged[m] {
            continue;
        }
        let key = merged.upper_bound(&(limit - merged[m]));
        if key == 0 {
            continue;
        }
        let val = merged[key - 1] + merged[m];
        if result < val {
            result = val;
        }
    }
    return result;
}

pub fn aggressive_cows(mut a: Vec<u32>, m: u32) -> u32 {
    a.sort(); // NlogN

    let mut left = 0;
    let mut right = std::u32::MAX;

    while right - left > 1 {
        let x = (left + right) / 2;
        let mut count = 1;
        let mut prev = 0;
        for i in 0..a.len() {
            if a[i] - a[prev] >= x {
                count += 1;
                prev = i;
            }
        }
        if count >= m {
            left = x;
        } else {
            right = x;
        }
    }
    return left as u32;
}

#[cfg(test)]
mod tests {
    use super::{aggressive_cows, darts, nth_rank, sum_of_three_sequences};

    #[test]
    fn test_nth_rank() {
        assert_eq!(nth_rank(vec![12, 43, 7, 15, 9]), vec![2, 4, 0, 3, 1]);
    }

    #[test]
    fn test_sum_of_three_sequenes() {
        assert_eq!(
            sum_of_three_sequences(vec![1, 5], vec![2, 4], vec![3, 6]),
            3
        );
        assert_eq!(
            sum_of_three_sequences(vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]),
            27
        );
    }

    #[test]
    fn test_darts() {
        assert_eq!(darts(vec![3, 14, 15, 9], 50), 48);
        assert_eq!(darts(vec![16, 11, 2], 21), 20);
    }

    #[test]
    fn test_aggressive_cows() {
        assert_eq!(aggressive_cows(vec![1, 2, 8, 4, 9], 3), 3);
    }
}
