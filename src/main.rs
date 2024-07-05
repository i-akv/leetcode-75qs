mod problems {
    pub mod _1;
}

use problems::_1::Solution;

fn main() {
    let word1 = String::from("abc");
    let word2 = String::from("pqrst");
    let s = Solution::merge_alternately(word1, word2);
    println!("{}", s);
}

// struct Solution{}
// impl Solution {
//     pub fn _fn(){}
// }