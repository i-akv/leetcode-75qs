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

#[test]
fn test_1(){
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let ans = Solution::is_subsequence(s, t);
    assert_eq!(true, ans);
}

#[test]
fn test_2(){
    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let ans = Solution::is_subsequence(s, t);
    assert_eq!(false, ans);
}