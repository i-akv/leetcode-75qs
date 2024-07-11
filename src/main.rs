mod problems {
    pub mod _7;
}
use problems::_7::Solution;

fn main() {
    let answer = Solution::product_except_self(vec![1,2,3,4]);
    println!("{:?}", answer);
}