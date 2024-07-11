pub struct Solution{}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut zeroes = 0;
        let mut product = 1;
        for num in nums.iter() {
            if *num==0 {zeroes+=1;continue;}
            product*=*num;
        }

        let mut product_vec = Vec::new();
        for num in nums.iter() {
            match zeroes {
                0 => {product_vec.push(product/(*num))},
                1 => {
                    match num {
                        0 => { product_vec.push(product) },
                        _ => {
                            product_vec.push(0);
                        }
                    }
                },
                _=>{product_vec.push(0)}
            }
        }

        product_vec
    }
}