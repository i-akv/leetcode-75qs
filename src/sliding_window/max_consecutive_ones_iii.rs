struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut lp = 0;
        let mut max_len = 0;
        let mut zeroes = 0;

        for rp in 0..nums.len() {
            if nums[rp] == 0 {zeroes+=1;}
            match zeroes<=k {
                true => {
                    max_len = max_len.max(rp-lp+1);
                },
                false => {
                    if nums[lp] == 0 {zeroes-=1;}
                    lp += 1;
                }
            }
        }

        max_len as i32
    }
}

#[test]
fn test_1() {
    let (nums, k) = (vec![1,1,1,0,0,0,1,1,1,1,0], 2);
    let output = 6;
    assert_eq!(Solution::longest_ones(nums, k), output);
}

#[test]
fn test_2() {
    let (nums, k) = (vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3);
    let output = 10;
    assert_eq!(Solution::longest_ones(nums, k), output);
}