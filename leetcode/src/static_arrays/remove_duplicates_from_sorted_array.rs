impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cur = 0;
        for i in 1..nums.len() {
            if nums[cur] != nums[i] {
                cur += 1;
                nums[cur] = nums[i];
            }
        }
        return (cur + 1) as i32;
    }
}
