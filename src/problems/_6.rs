use std::collections::VecDeque;
pub struct Solution{}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut stack: VecDeque<&str> = VecDeque::new();

        for word in s.trim().split_whitespace().collect::<Vec<&str>>().iter() {
            stack.push_back(word);
        }

        let mut output = String::new();
        while !stack.is_empty() {
            output.push_str(&stack.pop_back().unwrap().to_string());
            output.push(' ');
        }
        output[..output.len()-1].to_string()
    }
}