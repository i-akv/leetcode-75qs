pub struct Solution{}
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::new();
        let max_candies = candies.iter().max().unwrap();

        for candies in candies.iter() {
            if extra_candies+candies>=*max_candies { result.push(true) }
            else {result.push(false)};
        }

        result
    }
}