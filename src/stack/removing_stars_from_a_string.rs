struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::new();
        for char in s.chars() {
            match char {
                '*' => {
                    stack.pop();
                }
                _ => {
                    stack.push(char);
                }
            }
        }

        stack.iter().collect::<String>()
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::remove_stars("leet**cod*e".to_string()), "lecoe".to_string())
}

#[test]
fn test_2() {
    assert_eq!(Solution::remove_stars("erase*****".to_string()), "".to_string())
}