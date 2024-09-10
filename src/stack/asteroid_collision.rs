struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let (mut result, mut idx) = (vec![], 0);
        while idx < asteroids.len() {
            if !result.is_empty() {
                let current = result[result.len() - 1];
                if current > 0 && asteroids[idx] < 0 {
                    if i32::abs(current) <= i32::abs(asteroids[idx]) {
                        result.pop();
                        if i32::abs(current) == i32::abs(asteroids[idx]) {
                            idx += 1;
                        }
                    } else {
                        idx += 1;
                    }
                    continue;
                }
            }
            result.push(asteroids[idx]);
            idx += 1;
        }
        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10])
}

#[test]
fn test_2() {
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![])
}

#[test]
fn test_3() {
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10])
}