// 30. Substring with Concatenation of All Words

use std::collections::HashMap;

struct Solution{}impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let hash_map: HashMap<String, u8> = HashMap::new();

        vec![]
    }

    fn permutations(words: Vec<String>, index: usize) -> Vec<String> {
        if index > words.len()-1 {return vec![];}


        vec![]
    }
}

fn main() {
    let s = Solution::find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]);
    println!("{:?}", s);
}
