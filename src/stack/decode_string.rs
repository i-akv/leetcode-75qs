struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut slice = String::new();
        let mut stack = Vec::new();
        let mut num = String::new();
        for char in s.chars() {
            match char {
                '0'..='9' => num.push(char),
                'a'..='z' => slice.push(char),
                '[' => {
                    stack.push(num);
                    stack.push(slice);
                    slice = String::new();
                    num = String::new();
                }
                ']' => {
                    let (mut pop_slice, mul) = (stack.pop().unwrap(), stack.pop().unwrap().parse::<usize>().unwrap());
                    for _ in 0..mul { pop_slice.push_str(&slice) }
                    slice = pop_slice;
                }
                _ => {}
            }
        }

        slice
    }
}

#[test]
fn test_1() {
    let input = "3[a]2[bc]".to_string();
    let output = "aaabcbc".to_string();

    assert_eq!(output, Solution::decode_string(input));
}

#[test]
fn test_2() {
    let input = "3[a2[c]]".to_string();
    let output = "accaccacc".to_string();

    assert_eq!(output, Solution::decode_string(input));
}

#[test]
fn test_3() {
    let input = "2[abc]3[cd]ef".to_string();
    let output = "abcabccdcdcdef".to_string();

    assert_eq!(output, Solution::decode_string(input));
}
