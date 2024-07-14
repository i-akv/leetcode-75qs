pub struct Solution;
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut wp = 0;
        let mut char = chars[0];
        let mut count = 0;

        for i in chars.clone().iter() {
            if *i == char {count+=1; continue;}

            chars[wp] = char;
            wp+=1;

            if count==1 {

            } else {
                for c in count.to_string().chars() {
                    chars[wp] = c;
                    wp +=1 ;
                }
            }

            char = *i;
            count = 1;

        }

        chars[wp] = char;
        wp+=1;

        if count == 1 {
            wp as i32
        } else {
            for c in count.to_string().chars() {
                chars[wp] = c;
                wp+=1;
            }
            wp as i32
        }
    }
}