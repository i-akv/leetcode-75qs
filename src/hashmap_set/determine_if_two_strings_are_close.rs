use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() { return false; }

        let mut map1 = HashMap::new();
        for char in word1.chars() {
            *map1.entry(char).or_insert(0) += 1;
        }
        let keys1 = map1.iter().map(|(k, _)| *k).collect::<Vec<_>>();
        let mut freq1 = map1.iter().map(|(_, v)| v).collect::<Vec<_>>();
        freq1.sort_unstable();

        let mut map2 = HashMap::new();
        for char in word2.chars() {
            if !keys1.contains(&char) { return false; }
            *map2.entry(char).or_insert(0) += 1;
        }
        let mut freq2 = map2.iter().map(|(_, v)| v).collect::<Vec<_>>();
        freq2.sort_unstable();

        freq1 == freq2
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::close_strings("abc".to_string(), "bca".to_string()), true)
}

#[test]
fn test_2() {
    assert_eq!(Solution::close_strings("a".to_string(), "aa".to_string()), false)
}

#[test]
fn test_3() {
    assert_eq!(Solution::close_strings("cabbba".to_string(), "abbccc".to_string()), true)
}