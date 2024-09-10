use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occurence = vec![0u16; 2001];

        for num in arr.iter() {
            occurence[(*num + 1000) as usize] += 1;
        }

        let occurence = occurence.into_iter().filter(|&n| n != 0u16);
        occurence.clone().collect::<HashSet<_>>().len() == occurence.collect::<Vec<_>>().len()
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
}

#[test]
fn test_3() {
    assert_eq!(Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]), true);
}