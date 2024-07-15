pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if "" == s {return true;}

        let mut ptr = 0;
        let string_length = s.len();
        let s = s.chars().collect::<Vec<char>>();

        for char in t.chars() {
            if ptr == string_length {return true;}
            if char == s[ptr] {
                ptr+=1;
            }
        }
        ptr == string_length
    }
}