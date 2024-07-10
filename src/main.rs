mod problems {
    pub mod _6;
}
use problems::_6::Solution;
fn main() {
    let i = "  a     good      example   ";
    let s = Solution::reverse_words(i.to_string());
    println!("{}", s);
}
