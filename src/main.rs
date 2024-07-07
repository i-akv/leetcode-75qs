mod problems {
    pub mod _4;
}

use problems::_4::Solution;
fn main() {
let s = Solution::can_place_flowers(vec![1,0,0,0,1], 1);
    println!("{}", s);
}