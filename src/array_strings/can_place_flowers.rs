pub struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let mut remaining = n;

        for i in 0..flowerbed.len() {
            if remaining==0 {return true;}

            if (i == 0 || flowerbed[i - 1] == 0) && flowerbed[i] == 0 && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                remaining -= 1;
            }
        }

        remaining == 0
    }
}

#[test]
fn test_1(){
    assert_eq!(true, Solution::can_place_flowers([1,0,0,0,1].to_vec(), 1));
}

#[test]
fn test_2(){
    assert_eq!(false, Solution::can_place_flowers([1,0,0,0,1].to_vec(), 2));
}