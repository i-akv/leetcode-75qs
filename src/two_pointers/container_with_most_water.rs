pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut lp, mut rp, mut area) = (0, height.len()-1, 0 as i32);

        while lp < rp {
            let new_area = ((rp-lp) as i32)*(height[lp].min(height[rp]));

            if new_area > area {area = new_area}
            match height[lp] < height[rp] {
                true => {lp+=1;},
                false => {rp-=1;}
            }
        }

        area
    }
}

#[test]
fn test_1(){
    let height = [1,8,6,2,5,4,8,3,7].to_vec();
    let ans = Solution::max_area(height);
    assert_eq!(49, ans);
}

#[test]
fn test_2(){
    let height = [1,1].to_vec();
    let ans = Solution::max_area(height);
    assert_eq!(1, ans);
}