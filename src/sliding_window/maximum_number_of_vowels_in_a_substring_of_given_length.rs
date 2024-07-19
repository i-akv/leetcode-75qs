pub struct Solution;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max = 0;
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let chars = s.chars().collect::<Vec<char>>();

        let mut lp: usize = 0;
        let mut rp = 0;

        let mut count = 0;
        while rp < s.len() {
            if vowels.contains(&chars[rp]) {count+=1}

            if rp-lp+1 == k as usize {
                max = max.max(count);
                if vowels.contains(&chars[lp]) {count-=1;}
                lp+=1;
            }

            rp+=1;
        }

        max
    }
}

#[test]
fn test_1() {
    let s = "abciiidef".to_string();
    let k = 3;
    let ans = Solution::max_vowels(s, k);
    assert_eq!(3, ans);
}

#[test]
fn test_2(){
    let s = "aeiou".to_string();
    let k = 2;
    let ans = Solution::max_vowels(s, k);
    assert_eq!(2, ans);
}

#[test]
fn test_3() {
    let s = "leetcode".to_string();
    let k = 3;
    let ans = Solution::max_vowels(s, k);
    assert_eq!(2, ans);
}

#[test]
fn test_4() {
    let s = "ibpbhixfiouhdljnjfflpapptrxgcomvnb".to_string();
    let k = 33;
    let ans = Solution::max_vowels(s, k);
    assert_eq!(7, ans);
}

#[test]
fn test_5() {
    let s = "weallloveyou".to_string();
    let k = 7;
    let ans = Solution::max_vowels(s, k);
    assert_eq!(4, ans);
}