struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut cols = vec![vec![]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                cols[i].push(grid[j][i]);
            }
        }

        let mut count = 0i32;
        for col in cols.iter() {
            for row in grid.iter() {
                if row == col { count += 1; }
            }
        }

        count
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::equal_pairs([[3, 2, 1].to_vec(), [1, 7, 6].to_vec(), [2, 7, 7].to_vec()].to_vec()), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::equal_pairs([[3, 1, 2, 2].to_vec(), [1, 4, 4, 5].to_vec(), [2, 4, 2, 2].to_vec(), [2, 4, 2, 2].to_vec()].to_vec()), 3);
}
