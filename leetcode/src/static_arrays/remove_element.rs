impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        while let Some(index) = nums.iter().position(|v| *v == val) {
            nums.remove(index);
        }

        nums.len() as i32
    }
}
