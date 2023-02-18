use std::cmp::Ordering;

pub fn least_coin_usage(mut total: u32) -> u32 {
    let mut coins = vec![1, 5, 10, 50, 100, 500];
    coins.reverse();
    let mut result = 0;

    for coin in coins {
        let used = total / coin;
        total -= used * coin;
        result += used;
    }

    return result;
}

pub fn segment_scheduling(starts: Vec<u32>, ends: Vec<u32>) -> u32 {
    struct Job {
        start: u32,
        end: u32,
    }

    let mut jobs: Vec<Job> = vec![];
    for i in 0..starts.len() {
        jobs.push(Job {
            start: starts[i],
            end: ends[i],
        });
    }
    jobs.sort_by_key(|j| j.end);

    let mut result = 0;
    let mut current_end_time = 0;

    jobs.iter()
        .for_each(|job| match current_end_time.cmp(&job.start) {
            Ordering::Greater => return,
            _ => {
                result += 1;
                current_end_time = job.end;
            }
        });

    return result;
}

pub fn multiple_array(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let mut result = 0;
    for i in 0..a.len() {
        let manipulatable = a[i] + result;
        let remaining = manipulatable % b[i];
        if remaining != 0 {
            result += b[i] - remaining;
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::{least_coin_usage, multiple_array, segment_scheduling};
    #[test]
    fn test_least_coin_usage() {
        assert_eq!(least_coin_usage(666), 6);
        assert_eq!(least_coin_usage(363), 8);
    }

    #[test]
    fn test_segment_scheduling() {
        assert_eq!(segment_scheduling(vec![1, 4, 6], vec![5, 6, 8]), 2);
    }

    #[test]
    fn test_multiple_array() {
        assert_eq!(multiple_array(vec![3, 2, 9], vec![5, 7, 4]), 7);
    }
}
