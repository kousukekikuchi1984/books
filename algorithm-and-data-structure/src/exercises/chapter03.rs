pub fn found(nums: Vec<u32>, target_num: u32) -> usize {
    let mut found_id = 1;
    for (i, num) in nums.iter().enumerate() {
        if num == &target_num {
            found_id = i;
        }
    }
    return found_id;
}

pub fn found_counter(nums: Vec<u32>, target_num: u32) -> u32 {
    let mut count = 0;
    for num in nums {
        if num == target_num {
            count += 1
        }
    }
    return count;
}

pub fn second_min(nums: Vec<u32>) -> u32 {
    let mut first = std::u32::MAX;
    let mut second = std::u32::MAX;

    for num in nums {
        if num < first {
            second = first;
            first = num;
        } else if num < second {
            second = num
        }
    }
    return second;
}

pub fn max_distance(nums: Vec<u32>) -> u32 {
    let mut max = std::u32::MIN;
    let mut min = std::u32::MAX;

    for num in nums {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
    }
    return max.abs_diff(min);
}

pub fn divide_two(nums: Vec<u32>) -> u32 {
    let mut numbers = nums.to_vec();
    let mut j = 0;
    loop {
        for num in &mut numbers {
            if *num % 2 == 0 {
                *num /= 2;
                continue;
            } else {
                return j;
            }
        }
        j += 1;
    }
}

pub fn num_of_three_integers(k: u32, n: u32) -> u32 {
    let mut counter = 0;
    for x in 1..=k {
        for y in 1..=k {
            let z = n - (x + y);
            if z <= k {
                counter += 1;
            }
        }
    }
    return counter;
}

pub fn sum_of_multiple_sum(s: String) -> u32 {
    let length = s.len() - 1;
    let mut result: u32 = 0;
    let chars: Vec<char> = s.chars().collect();

    for bit in 0..1 << length {
        let mut sum = 0;
        let mut num = chars[0].to_digit(10).unwrap() as u32;

        for i in 0..length {
            if bit & (1 << i) != 0 {
                sum += num;
                num = 0;
            }
            num = num * 10 + chars[i + 1 as usize].to_digit(10).unwrap() as u32;
        }
        sum += num;
        result += sum;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::{
        divide_two, found, found_counter, max_distance, num_of_three_integers, second_min,
        sum_of_multiple_sum,
    };
    #[test]
    fn test_found() {
        assert_eq!(found(vec![1, 2, 3, 1, 2], 1), 3);
        assert_eq!(found(vec![1, 2, 3, 1, 2], 2), 4);
    }

    #[test]
    fn test_found_counter() {
        assert_eq!(found_counter(vec![1, 2, 3, 1, 2], 1), 2);
        assert_eq!(found_counter(vec![1, 2, 3, 1, 2], 3), 1);
    }

    #[test]
    fn test_second_min() {
        assert_eq!(second_min(vec![1, 2, 3, 4, 5, 6]), 2);
        assert_eq!(second_min(vec![1, 3, 4, 2, 6, 5]), 2);
    }

    #[test]
    fn test_max_distance() {
        assert_eq!(max_distance(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_distance(vec![5, 0, 3, 4, 2]), 5);
    }

    #[test]
    fn test_devide_two() {
        assert_eq!(divide_two(vec![2, 4, 6, 8]), 1);
        assert_eq!(divide_two(vec![8, 16, 32, 64]), 3);
    }

    #[test]
    fn test_num_of_three_integers() {
        assert_eq!(num_of_three_integers(3, 9), 1);
        assert_eq!(num_of_three_integers(3, 8), 3);
    }

    #[test]
    fn test_sum_of_multiple_sum() {
        assert_eq!(sum_of_multiple_sum(String::from("125")), 176);
    }
}
