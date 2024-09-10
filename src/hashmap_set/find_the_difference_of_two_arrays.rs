use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1 = nums1.into_iter().collect::<HashSet<i32>>();
        let nums2 = nums2.into_iter().collect::<HashSet<i32>>();
        vec![
            nums1.clone().into_iter().filter(|n| !nums2.contains(n)).collect::<Vec<i32>>(),
            nums2.into_iter().filter(|n| !nums1.contains(n)).collect::<Vec<i32>>(),
        ]
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]), vec![vec![1, 3], vec![6, 4]])
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]), vec![vec![3], vec![]])
}