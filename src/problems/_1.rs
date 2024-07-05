use std::cmp::max;
pub struct Solution {}
#[warn(dead_code)]
impl Solution {
    pub fn merge_alternately_bf(word1:String, word2:String) -> String {
        let mut result = String::new();

        for i in 0..max(word1.len(), word2.len()) {
            let char1 = word1.chars().nth(i);
            match char1 {
                None => {}
                Some(c) => {
                    result.push(c)
                }
            }

            let char2 = word2.chars().nth(i);
            match char2 {
                None => {}
                Some(c) => {
                    result.push(c)
                }
            }
        }
        result
    }

    pub fn merge_alternately(word1:String, word2:String) -> String {
        let mut result = String::new();
        let chars1 =word1.chars();
        let mut chars2 = word2.chars().peekable();

        for char in chars1 {
            result.push(char);
            if let Some(c) = chars2.next() {
                result.push(c);
            }
        }

        for char in chars2 {
            result.push(char);
        }

        result
    }
}