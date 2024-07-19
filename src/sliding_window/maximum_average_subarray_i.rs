pub struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        for i in 0..k as usize {
            sum += nums[i];
        };

        let mut max_sum = sum;
        for i in 1..=nums.len()-k as usize {
            sum -= nums[i-1];
            sum += nums[i+k as usize-1];

            max_sum = sum.max(max_sum);
        }

        max_sum as f64/k as f64
    }
}

#[test]
fn test_1() {
    let nums = [1,12,-5,-6,50,3].to_vec();
    let k = 4;

    let ans = Solution::find_max_average(nums, k);
    assert_eq!(12.75000, ans);
}

#[test]
fn test_2() {
    let nums = [5].to_vec();
    let k = 1;

    let ans = Solution::find_max_average(nums, k);
    assert_eq!(5.00000, ans);
}