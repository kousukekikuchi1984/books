fn summer_holiday_happiness(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
    // a -> happiness of swimming
    // b -> happiness of bug catching
    // c -> happiness of doing homework
    // n -> n days
    // same category cannot be used consequcutive days
    let values = vec![&a, &b, &c];
    let mut dp = vec![vec![std::i32::MIN; 3]; a.len()];
    for i in 0..a.len() {
        for j in 0..3 {
            if i == 0 {
                dp[i][j] = values[j][i];
            } else {
                // remove same index;
                let mut val = values[j].to_vec();
                val.remove(i);
                let happiness = val.iter().max().unwrap();
                dp[i][j] = dp[i - 1][j] + happiness;
            }
        }
    }
    return *dp[a.len() - 1].iter().max().unwrap();
}

fn subset_sum(nums: Vec<i32>, w: i32) -> bool {
    let mut dp = vec![vec![0; w as usize + 1]; nums.len()];

    for n in 0..nums.len() {
        for val in 0..w as usize {
            if n == 0 {
                if val == 0 {
                    dp[n][val] = 1;
                } else if val == nums[n] as usize {
                    dp[n][val] = 1;
                }
            } else {
                if dp[n - 1][val] == 1 {
                    dp[n][val] = 1;
                    let selected = dp[n - 1][val] + nums[n];
                    if selected <= w {
                        dp[n][selected as usize] = 1;
                    }
                }
            }
        }
    }
    dp[nums.len() - 1][w as usize] == 1
}

fn subset_sum_times(nums: Vec<i32>, w: i32) -> i32 {
    let mut dp = vec![vec![0; w as usize + 1]; nums.len()];
    for n in 0..nums.len() {
        for val in 0..=w as usize {
            if n == 0 {
                if val == 0 {
                    dp[n][val] = 1;
                } else if val == nums[n] as usize {
                    dp[n][val] = 1;
                }
            } else {
                if dp[n - 1][val] > 0 {
                    dp[n][val] = dp[n - 1][val] + dp[n][val];

                    let selected = val + nums[n] as usize;
                    if selected <= w as usize {
                        dp[n][selected] += 1;
                    }
                }
            }
        }
    }

    return dp[nums.len() - 1][w as usize];
}

fn subset_sum_multiple_times(nums: Vec<i32>, w: i32, k: i32) -> bool {
    let mut dp = vec![vec![std::i32::MAX; w as usize + 1]; nums.len()];

    for n in 0..nums.len() {
        for val in 0..=w as usize {
            if n == 0 {
                if val == 0 {
                    dp[n][val] = 0;
                } else if val == nums[n] as usize {
                    dp[n][val] = 1;
                }
            } else {
                if dp[n - 1][val] != std::i32::MAX {
                    dp[n][val] = dp[n - 1][val];

                    let selected = val + nums[n] as usize;
                    if selected <= w as usize {
                        dp[n][selected] = {
                            if dp[n][selected] == std::i32::MAX {
                                1
                            } else {
                                dp[n][selected] + 1
                            }
                        }
                    }
                }
            }
        }
    }

    return dp[nums.len() - 1][w as usize] <= k;
}

fn subset_sum_unlimited(nums: Vec<u32>, w: u32) -> bool {
    let mut dp = vec![vec![0; w as usize + 1]; nums.len()];

    for n in 0..nums.len() {
        for val in 0..=w as usize {
            if n == 0 {
                let mut i = 0;
                while i * nums[n] <= w {
                    let score = i * nums[n];
                    dp[n][score as usize] = 1;
                    i += 1;
                }
            } else {
                if dp[n - 1][val] > 0 {
                    let mut i = 0;
                    while (i * nums[n]) + val as u32 <= w {
                        let score = (i * nums[n]) as usize + val;
                        dp[n][score as usize] = dp[n - 1][val] + 1;
                        i += 1;
                    }
                }
            }
        }
    }
    return dp[nums.len() - 1][w as usize] > 0;
}

fn subset_sum_limited_times(nums: Vec<u32>, w: u32, m: u32) -> bool {
    let mut dp = vec![vec![0; w as usize + 1]; nums.len()];

    for n in 0..nums.len() {
        for val in 0..=w as usize {
            if n == 0 {
                let mut i = 0;
                while i * nums[n] <= w && i <= m {
                    let score = i * nums[n];
                    dp[n][score as usize] = 1;
                    i += 1;
                }
            } else {
                if dp[n - 1][val] > 0 {
                    let mut i = 0;
                    while (i * nums[n]) + val as u32 <= w && i <= m {
                        let score = (i * nums[n]) as usize + val;
                        dp[n][score as usize] = dp[n - 1][val] + 1;
                        i += 1;
                    }
                }
            }
        }
    }
    return dp[nums.len() - 1][w as usize] > 0;
}

fn longest_common_sequence(s: &str, v: &str) -> u32 {
    let mut dp = vec![vec![0; v.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..v.len() {
            let schar = s.chars().collect::<Vec<char>>()[i];
            let vchar = v.chars().collect::<Vec<char>>()[j];

            if schar == vchar {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                if dp[i][j + 1] > dp[i + 1][j] {
                    dp[i + 1][j + 1] = dp[i][j + 1];
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j];
                }
            }
        }
    }
    return dp[s.len()][v.len()];
}

fn biggest_sum_of_average_partitions(n: Vec<u32>, m: u32) -> f64 {
    // calculate average distance between i, j
    let mut f = vec![vec![0.; n.len() + 1]; n.len() + 1];
    for i in 1..=n.len() {
        let mut j = 0;
        while j < i {
            let mut sum = 0.;
            let mut k = j;
            while k < i {
                sum += n[k] as f64;
                k += 1;
            }
            f[j][i] = sum / (i - j) as f64;
            j += 1;
        }
    }

    //
    let mut dp = vec![vec![std::f64::MIN; m as usize + 1]; n.len() + 1];
    dp[0][0] = 0.;
    for i in 0..=n.len() {
        let mut j = 0;
        while j < i {
            for k in 1..=m as usize {
                let score = dp[j][k - 1] + f[j][i];
                if dp[i][k] < score {
                    dp[i][k] = score;
                }
            }
            j += 1;
        }
    }

    return dp[n.len()][m as usize];
}

#[cfg(test)]
mod tests {
    use super::{
        biggest_sum_of_average_partitions, longest_common_sequence, subset_sum,
        subset_sum_limited_times, subset_sum_multiple_times, subset_sum_times,
        subset_sum_unlimited, summer_holiday_happiness,
    };

    #[test]
    fn test_summer_holiday_happiness() {
        let bugs = vec![1, 2, 3];
        let swimming = vec![2, 3, 1];
        let homework = vec![1, 1, 1];
        assert_eq!(summer_holiday_happiness(swimming, bugs, homework), 7);
    }

    #[test]
    fn test_subset_sum() {
        assert_eq!(subset_sum(vec![1, 3, 8], 9), true);
        assert_eq!(subset_sum(vec![1, 3, 7], 9), false);
    }

    #[test]
    fn test_subset_sum_times() {
        assert_eq!(subset_sum_times(vec![1, 3, 8], 9), 1);
        assert_eq!(subset_sum_times(vec![1, 3, 8, 6], 9), 2);
    }

    #[test]
    fn test_subset_sum_multiple_times() {
        assert_eq!(subset_sum_multiple_times(vec![1, 3, 5, 6], 9, 2), true);
        assert_eq!(subset_sum_multiple_times(vec![1, 3, 5, 4], 12, 3), true);
        assert_eq!(subset_sum_multiple_times(vec![1, 3, 5, 4], 11, 3), false);
    }

    #[test]
    fn test_subset_sum_unlimited() {
        assert_eq!(subset_sum_unlimited(vec![1, 3, 5, 7], 31), true); // 1 * 31
        assert_eq!(subset_sum_unlimited(vec![3, 5, 7], 31), true); // 7 * 3 + 5 * 2
        assert_eq!(subset_sum_unlimited(vec![3, 5, 7], 13), true); // 3 + 3 + 7
        assert_eq!(subset_sum_unlimited(vec![2, 8, 12], 31), false); // even number cannot create odd nuber
    }

    #[test]
    fn test_subset_sum_limited_times() {
        assert_eq!(subset_sum_limited_times(vec![3, 5, 7], 31, 3), true);
        assert_eq!(subset_sum_limited_times(vec![1, 3, 5, 7], 35, 3), true);
        assert_eq!(subset_sum_limited_times(vec![1, 3, 5, 7], 35, 2), false);
    }

    #[test]
    fn test_longest_common_sequence() {
        assert_eq!(longest_common_sequence("ABC", "AEBICD"), 3);
        assert_eq!(longest_common_sequence("DAC", "AEBICD"), 2);
    }

    #[test]
    fn test_biggest_sum_of_average_partitions() {
        assert_eq!(
            biggest_sum_of_average_partitions(vec![9, 1, 2, 3, 9], 3),
            20.
        );
        assert_eq!(biggest_sum_of_average_partitions(vec![14, 4, 9, 7], 1), 8.5);
    }
}
