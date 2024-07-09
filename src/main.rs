mod problems {
    pub mod _5;
}

use problems::_5::Solution;
fn main() {
    let s = Solution::reverse_vowels("a.".to_string());
    println!("{}", s);
}
