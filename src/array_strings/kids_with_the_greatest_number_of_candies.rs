pub struct Solution;
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

#[test]
fn test_1() {
    let candies = [2,3,5,1,3].to_vec();
    let ans = Solution::kids_with_candies(candies, 3);
    assert_eq!([true,true,true,false,true].to_vec(), ans);
}

#[test]
fn test_2() {
    let candies = [4,2,1,1,2].to_vec();
    let ans = Solution::kids_with_candies(candies, 1);
    assert_eq!([true,false,false,false,false].to_vec(), ans);
}

#[test]
fn test_3() {
    let candies = [12,1,12].to_vec();
    let ans = Solution::kids_with_candies(candies, 10);
    assert_eq!([true,false,true].to_vec(), ans);
}