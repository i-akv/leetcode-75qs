mod problems {
    pub mod _11;
}

use problems::_11::Solution;

fn main() {
    let ans = Solution::is_subsequence(String::from("abc"), String::from("adbgce"));
    println!("{}", ans);
}