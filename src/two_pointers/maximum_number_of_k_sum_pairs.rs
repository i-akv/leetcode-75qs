use std::cmp::Ordering;
pub struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let (mut lp, mut rp, mut count) = (0, nums.len()-1, 0);

        let mut nums = nums;
        nums.sort_unstable();

        while lp<rp {
            match (nums[lp]+nums[rp]).cmp(&k) {
                Ordering::Equal => {lp+=1; rp-=1; count+=1;},
                Ordering::Less => lp+=1,
                Ordering::Greater => rp-=1
            }
        }

        count
    }
}

#[test]
fn test_1(){
    let nums = [1,2,3,4].to_vec();
    let k = 5;
    let ans = Solution::max_operations(nums, k);
    assert_eq!(2, ans);
}

#[test]
fn test_2(){
    let nums = [3,1,3,4,3].to_vec();
    let k = 6;
    let ans = Solution::max_operations(nums, k);
    assert_eq!(1, ans);
}