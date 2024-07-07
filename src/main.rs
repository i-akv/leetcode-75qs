mod problems {
    pub mod _3;
}

use problems::_3::Solution;
fn main() {
let s = Solution::kids_with_candies(vec![4,2,1,1,2], 1);
    println!("{:?}", s);
}