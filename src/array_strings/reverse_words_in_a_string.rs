pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut stack = std::collections::VecDeque::new();

        s.trim().split_whitespace().for_each(|word| stack.push_back(word));

        let mut output = String::new();
        while !stack.is_empty() {
            output.push_str(&stack.pop_back().unwrap().to_string());
            output.push(' ');
        }
        output[..output.len()-1].to_string()
    }
}

#[test]
fn test_1(){
    let s = "the sky is blue".to_string();
    let ans = Solution::reverse_words(s);
    assert_eq!("blue is sky the", ans);
}

#[test]
fn test_2(){
    let s = "  hello world  ".to_string();
    let ans = Solution::reverse_words(s);
    assert_eq!("world hello", ans);
}