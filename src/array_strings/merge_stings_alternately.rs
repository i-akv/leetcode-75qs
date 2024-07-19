pub struct Solution;
impl Solution {
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

#[test]
fn test_1() {
    let ans = Solution::merge_alternately("abc".to_string(), "pqr".to_string());
    assert_eq!("apbqcr", ans);
}

#[test]
fn test_2() {
    let ans = Solution::merge_alternately("ab".to_string(), "pqrs".to_string());
    assert_eq!("apbqrs", ans);
}

#[test]
fn test_3() {
    let ans = Solution::merge_alternately("abcd".to_string(), "pq".to_string());
    assert_eq!("apbqcd", ans);
}