mod problems {
    pub mod _8;
}
use problems::_8::Solution;

fn main() {
    let v = vec![20,100,10,12,5,13];
    let ans = Solution::increasing_triplet(v);
    println!("{}", ans);
}