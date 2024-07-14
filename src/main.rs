mod problems {
    pub mod _10;
}

use problems::_10::Solution;

fn main() {
    let mut vec = vec![0,1,0,3,12];
    let ans = Solution::move_zeroes(&mut vec);
    println!("{:?}", vec);
}