pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut lp = 0;
        let mut max_len = 0;
        let mut zeroes = 0;
        for rp in 0..nums.len() {
            if nums[rp] == 0 {zeroes+=1;}
            match zeroes <= 1 {
                true => max_len = max_len.max(rp-lp+1),
                false => {
                    if nums[lp] == 0 {zeroes-=1;}
                    lp+=1;
                }
            }
        }

        if max_len != 0 {max_len as i32 - 1} else {0_i32}
    }
}

#[test]
fn test_1() {
    let nums = vec![1, 1, 0, 1];
    let output = 3;
    assert_eq!(Solution::longest_subarray(nums), output);
}

#[test]
fn test_2() {
    let nums = vec![0,1,1,1,0,1,1,0,1];
    let output = 5;
    assert_eq!(Solution::longest_subarray(nums), output);
}

#[test]
fn test_3() {
    let nums = vec![1,1,1];
    let output = 2;
    assert_eq!(Solution::longest_subarray(nums), output);
}