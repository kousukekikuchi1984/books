pub fn making_pairs(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let mut mut_a = a.to_vec();
    let mut mut_b = b.to_vec();
    mut_a.sort();
    mut_b.sort();

    let mut result = 0;
    let mut cur_to_a: usize = 0;
    for i in 0..mut_b.len() {
        if mut_b[i] > mut_a[cur_to_a] {
            result += 1;
            cur_to_a += 1;
        }
    }

    return result;
}

#[cfg(test)]

mod test {
    use super::making_pairs;

    #[test]
    fn test_making_pairs() {
        assert_eq!(making_pairs(vec![2, 6, 8, 14, 17], vec![1, 3, 5, 8, 9]), 3);
    }
}
