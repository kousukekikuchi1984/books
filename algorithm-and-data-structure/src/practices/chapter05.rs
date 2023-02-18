pub fn frog(costs: Vec<u32>) -> u32 {
    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;

    for i in 1..costs.len() {
        let cost = i32::abs(costs[i] as i32 - costs[i - 1] as i32) as u32 + cost_table[i - 1];
        if cost < cost_table[i] {
            cost_table[i] = cost;
        }
        if i > 1 {
            let cost = (costs[i] as i32 - costs[i - 2] as i32).abs() as u32 + cost_table[i - 2];
            if cost < cost_table[i] {
                cost_table[i] = cost;
            }
        }
    }

    return cost_table[cost_table.len() - 1];
}

pub fn frog_relaxsation(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }

    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;

    for i in 1..costs.len() {
        let cost = i32::abs(costs[i] as i32 - costs[i - 1] as i32) as u32 + cost_table[i - 1];
        relaxzation(&mut cost_table[i], cost);
        if i > 1 {
            let cost = (costs[i] as i32 - costs[i - 2] as i32).abs() as u32 + cost_table[i - 2];
            relaxzation(&mut cost_table[i], cost);
        }
    }
    return cost_table[cost_table.len() - 1];
}

pub fn frog_distribute(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }

    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;
    let len = costs.len();

    for i in 0..costs.len() - 2 {
        if i + 1 < len {
            let cost = i32::abs(costs[i + 1] as i32 - costs[i] as i32) as u32 + cost_table[i];
            relaxzation(&mut cost_table[i + 1], cost);
        }

        if i + 2 < len {
            let cost = i32::abs(costs[i + 2] as i32 - costs[i] as i32) as u32 + cost_table[i];
            relaxzation(&mut cost_table[i + 2], cost)
        }
    }
    return cost_table[cost_table.len() - 1];
}

pub fn frog_naive_recursive(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }

    fn _frog(i: usize, costs: &Vec<u32>) -> u32 {
        if i == 0 {
            return 0;
        }
        let mut res = std::u32::MAX;

        // from i - 1
        relaxzation(
            &mut res,
            _frog(i - 1, costs) + (costs[i] as i32 - costs[i - 1] as i32).abs() as u32,
        );

        // from i - 2
        if i > 2 {
            relaxzation(
                &mut res,
                _frog(i - 2, costs) + (costs[i] as i32 - costs[i - 2] as i32).abs() as u32,
            );
        }
        return res;
    }

    return _frog(costs.len() - 1, &costs);
}

pub fn frog_recursive(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }
    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;

    fn _frog(i: usize, costs: &Vec<u32>, cost_table: &mut Vec<u32>) -> u32 {
        if i == 0 {
            return 0;
        }
        if cost_table[i] < std::u32::MAX {
            return cost_table[i];
        }
        let mut res = std::u32::MAX;

        relaxzation(
            &mut res,
            _frog(i - 1, costs, cost_table) + (costs[i] as i32 - costs[i - 1] as i32).abs() as u32,
        );

        if i > 1 {
            relaxzation(
                &mut res,
                _frog(i - 2, costs, cost_table)
                    + (costs[i] as i32 - costs[i - 2] as i32).abs() as u32,
            );
        }

        cost_table[i] = res;
        return res;
    }

    return _frog(costs.len() - 1, &costs, &mut cost_table);
}

pub fn knapsack(weights: Vec<u32>, values: Vec<u32>, max_weight: u32, n: usize) -> u32 {
    let maxw: u32 = weights.iter().sum();

    let mut cost_table = vec![vec![0; maxw as usize]; n + 10];
    for i in 0..n {
        for w in 0..maxw {
            if w >= weights[i] {
                cost_table[i + 1][w as usize] = u32::max(
                    cost_table[i][w as usize],
                    cost_table[i][(w - weights[i]) as usize] + values[i],
                );
            } else {
                cost_table[i + 1][w as usize] = cost_table[i][w as usize];
            }
        }
    }
    return cost_table[n][max_weight as usize];
}

pub fn edit_distance(from: &str, to: &str) -> u32 {
    fn relaxzation(a: &mut u32, b: u32) {
        if *a > b {
            *a = b;
        }
    }

    let fchars: Vec<char> = from.chars().collect();
    let tchars: Vec<char> = to.chars().collect();
    let mut dp = vec![vec![std::u32::MAX; tchars.len() + 1]; fchars.len() + 1];
    dp[0][0] = 0;

    for i in 0..=fchars.len() {
        for j in 0..=tchars.len() {
            // manipulate edit
            if i > 0 && j > 0 {
                if fchars[i - 1] == tchars[j - 1] {
                    let prev = dp[i - 1][j - 1];
                    relaxzation(&mut dp[i][j], prev);
                } else {
                    let prev = dp[i - 1][j - 1] + 1;
                    relaxzation(&mut dp[i][j], prev);
                }
            }

            // deletion
            if i > 0 {
                let prev = dp[i - 1][j] + 1;
                relaxzation(&mut dp[i][j], prev);
            }

            // insertion
            if j > 0 {
                let prev = dp[i][j - 1] + 1;
                relaxzation(&mut dp[i][j], prev);
            }
        }
    }
    return dp[fchars.len()][tchars.len()];
}

pub fn partition(c: Vec<Vec<u32>>) -> u32 {
    fn relaxzation(a: &mut u32, b: u32) {
        if *a > b {
            *a = b;
        }
    }

    let mut dp = vec![std::u32::MAX; c.len()];
    dp[0] = 0;
    for i in 0..c.len() {
        let mut j = 0;
        while j < i {
            let calc = dp[j] + c[j][i];
            relaxzation(&mut dp[i], calc);
            j += 1;
        }
    }

    return dp[c.len() - 1];
}

#[cfg(test)]
mod tests {

    use super::{
        edit_distance, frog, frog_distribute, frog_naive_recursive, frog_recursive,
        frog_relaxsation, knapsack, partition,
    };

    #[test]
    fn test_frog() {
        assert_eq!(frog(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_relaxsation() {
        assert_eq!(frog_relaxsation(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog_relaxsation(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_distribute() {
        assert_eq!(frog_distribute(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog_distribute(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_naive_recursive() {
        assert_eq!(frog_naive_recursive(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog_naive_recursive(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_recursive() {
        assert_eq!(frog_recursive(vec![1, 3, 2, 4]), 3);
        assert_eq!(frog_recursive(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_knapsack() {
        let weights = vec![2, 1, 3, 2, 1, 5];
        let values = vec![3, 2, 6, 1, 3, 85];
        let max_weight = 9;
        let n = weights.len();
        assert_eq!(knapsack(weights, values, max_weight, n), 94);
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("logistic", "algorithm"), 6);
    }

    #[test]
    fn test_partition() {
        let c = vec![vec![1, 2, 3], vec![2, 3, 4], vec![1, 3, 2]];
        assert_eq!(partition(c), 3);
    }
}
