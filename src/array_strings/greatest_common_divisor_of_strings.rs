pub struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let s1 = str1.clone() + &str2.clone();
        let s2 = str2.clone() + &str1.clone();

        if s1==s2 {
            return str1[..Self::gcd_of_nums(str1.len() as u32, str2.len() as u32) as usize].to_string()
        }

        "".to_string()
    }

    fn gcd_of_nums(a: u32, b: u32) -> u32 {
        if b == 0 {
            a
        } else {
            Self::gcd_of_nums(b, a % b)
        }
    }
}

#[test]
fn test_1() {
    let ans = Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
    assert_eq!("ABC", ans);
}

#[test]
fn test_2() {
    let ans = Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
    assert_eq!("AB", ans);
}

#[test]
fn test_3() {
    let ans = Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string());
    assert_eq!("", ans);
}