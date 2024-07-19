pub struct Solution;
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut wp = 0;
        let mut char = chars[0];
        let mut count = 0;

        for i in chars.clone().iter() {
            if *i == char {count+=1; continue;}
            chars[wp] = char; wp+=1;
            if count!=1 {
                for c in count.to_string().chars() {
                    chars[wp] = c;
                    wp +=1 ;
                }
            }

            char = *i; count = 1;

        }

        chars[wp] = char; wp+=1;

        if count == 1 {
            wp as i32
        } else {
            for c in count.to_string().chars() {
                chars[wp] = c;
                wp+=1;
            }
            wp as i32
        }}
}

#[test]
fn test_1(){
    let mut chars = ['a','a','b','b','c','c','c'].to_vec();
    let ans = Solution::compress(&mut chars);
    assert_eq!(6, ans);
    assert_eq!(chars, ['a','2','b','2','c','3', 'c'].to_vec());
}

#[test]
fn test_2(){
    let mut chars = ['a'].to_vec();
    let ans = Solution::compress(&mut chars);
    assert_eq!(1, ans);
    assert_eq!(chars, ['a'].to_vec());
}

#[test]
fn test_3(){
    let mut chars = ['a','b','b','b','b','b','b','b','b','b','b','b','b'].to_vec();
    let ans = Solution::compress(&mut chars);
    assert_eq!(4, ans);
    assert_eq!(chars, ['a','b','1', '2','b','b','b','b','b','b','b','b','b'].to_vec());
}
