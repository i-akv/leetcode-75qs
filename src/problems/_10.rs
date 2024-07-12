pub struct Solution{}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len()!=1 {
            let mut lp = 0;
            let mut rp = 1;

            while None != nums.get(rp) {
                match nums[lp] {
                    0 => {
                        match nums[rp] {
                            0 => { rp += 1 },
                            _ => {
                                nums[lp] = nums[rp];
                                nums[rp] = 0;
                                lp+=1; rp+=1;
                            }
                        }
                    },
                    _ => {
                        lp+=1;
                        rp+=1;
                    }
                }
            }

        }
    }
}