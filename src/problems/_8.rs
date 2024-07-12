pub struct Solution{}
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut left = i32::MAX;
        let mut middle = i32::MAX;

        for num in nums.iter() {
            if middle<*num {return true;}
            if left<*num {middle = middle.min(*num)}
            left = left.min(*num);
        }

        false
    }
}