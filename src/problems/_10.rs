pub struct Solution{}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 0 {return;}

        let mut lp = 0;
        for mut rp in 1..nums.len() {
            if nums[lp] == 0 && nums[rp] ==0 {rp+=1}
            else if nums[lp] ==0 && nums[rp] != 0 {
                nums[lp] = nums[rp];
                nums[rp] = 0;
                lp+=1;
            }
            else {lp+=1}
        }
    }
}