pub struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len()==0 {return;}

        let mut lp = 0;
        for rp in 1..nums.len() {
            if nums[lp] == 0 && nums[rp] == 0 {}
            else if nums[lp] ==0 && nums[rp] != 0 {
                nums[lp] = nums[rp];
                nums[rp] = 0;
                lp+=1;
            }
            else {lp+=1}
        }
    }
}

#[test]
fn test_1(){
    let mut nums = [0,1,0,3,12].to_vec();
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [1,3,12,0,0].to_vec());
}

#[test]
fn test_2(){
    let mut nums = [0].to_vec();
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [0].to_vec());
}