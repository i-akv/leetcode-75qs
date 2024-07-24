pub struct Solution;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut sum = 0;

        gain.iter().for_each(|g| {
            sum += *g;
            if sum>max {max=sum}
        });

        max
    }
}

#[test]
fn test_1() {
    let gains = vec![-5,1,5,0,-7];
    let output = 1;
    assert_eq!(Solution::largest_altitude(gains), output);
}

#[test]
fn test_2() {
    let gains = vec![-4,-3,-2,-1,4,3,2];
    let output = 0;
    assert_eq!(Solution::largest_altitude(gains), output);
}
