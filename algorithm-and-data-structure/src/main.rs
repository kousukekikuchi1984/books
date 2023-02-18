mod exercises;
mod practices;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut matched = 0;
        for cur in 0..nums.len() {
            if nums[cur] != val {
                nums[matched] = nums[cur];
                matched += 1;
            }
        }
        return matched as i32;
    }
}

fn main() {
    println!("Hello, world!");
}
