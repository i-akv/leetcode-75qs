pub struct Solution{}

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