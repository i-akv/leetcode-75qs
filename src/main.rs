mod problems {
    pub mod _2;
}

use problems::_2::Solution;

fn main() {
    let word1 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXX");
    let word2 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX");
    let s = Solution::gcd_of_strings(word1, word2);
    println!("{}", s);

    // let n = Solution::kids_with_candies(vec![1, 2, 3], 12);
    // println!("N: {:?}", n);
}

// pub struct Solution{}
// impl Solution {
//     pub fn _fn(){}
// }