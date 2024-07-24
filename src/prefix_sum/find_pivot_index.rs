struct Solution;
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut lsum = vec![0; nums.len()];
        let mut rsum = vec![0; nums.len()];

        for i in 1..len {
            lsum[i] += lsum[i-1] + nums[i-1];
            rsum[len-1-i] += rsum[len-i] + nums[len-i];
        }

        for i in 0..len {
            if lsum[i] == rsum[i] {return i as i32;}
        }

        return -1;
    }

    pub fn pivot_index_lil_fast(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let total_sum: i32 = nums.iter().sum();

        for (middle_index, middle_val) in nums.iter().enumerate() {
            let right_sum  = total_sum - (middle_val + left_sum);
            if left_sum == right_sum {
                return middle_index as i32;
            }
            left_sum += middle_val;
        }
        -1
    }
}

#[test]
fn test_1() {
    let nums = vec![1,7,3,6,5,6];
    let output = 3;
    assert_eq!(Solution::pivot_index(nums), output);
}

#[test]
fn test_2() {
    let nums = vec![1,2,3];
    let output = -1;
    assert_eq!(Solution::pivot_index(nums), output);
}

#[test]
fn test_3() {
    let nums = vec![2,1,-1];
    let output = 0;
    assert_eq!(Solution::pivot_index(nums), output);
}