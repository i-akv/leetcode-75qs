pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut product = vec![1; len];

        for i in 1..len {
            product[i] = product[i-1]*nums[i-1];
        }

        let mut rp = 1;
        for i in (0..len-1).rev() {
            rp *= nums[i+1];
            product[i] *= rp;
        }

        product
    }
}

#[test]
fn test_1() {
    let nums = [1,2,3,4].to_vec();
    let ans = Solution::product_except_self(nums);
    assert_eq!(ans, [24, 12, 8, 6].to_vec());
}

#[test]
fn test_2() {
    let nums = [-1,1,0,-3,3].to_vec();
    let ans = Solution::product_except_self(nums);
    assert_eq!(ans, [0,0,9,0,0].to_vec());
}