pub struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut left = i32::MAX;
        let mut middle = i32::MAX;

        for num in nums.iter() {
            if middle<*num {return true;}
            if left<*num {middle = middle.min(*num);}
            left = left.min(*num);
        }

        false
    }
}

#[test]
fn test_1(){
    let nums = [1,2,3,4,5].to_vec();
    let ans = Solution::increasing_triplet(nums);
    assert_eq!(ans, true);
}

#[test]
fn test_2(){
    let nums = [5,4,3,2,1].to_vec();
    let ans = Solution::increasing_triplet(nums);
    assert_eq!(ans, false);
}

#[test]
fn test_3(){
    let nums = [2,1,5,0,4,6].to_vec();
    let ans = Solution::increasing_triplet(nums);
    assert_eq!(ans, true);
}