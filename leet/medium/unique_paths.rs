//https://leetcode.com/problems/unique-paths-ii/
pub struct Solution {}

const OBSTACLE: i32 = 1;
const SPACE: i32 = 0;

use std::collections::VecDeque;



impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {

        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();

        let mut n_paths = obstacle_grid.clone();
        for i in 0..n{
            for j in 0..m{
                n_paths[i][j] = 0;
            }
        }
        n_paths[0][0] = {if obstacle_grid[0][0] == SPACE {1} else {0}};

        let mut queue = VecDeque::new();
        if m > 1 {queue.push_back(  (0, 1) );}
        if n > 1 {queue.push_back(  (1, 0) );}
        while let Some( (x, y) ) = queue.pop_front() {
            if n_paths[x][y] != 0 {continue;}
            if obstacle_grid[x][y] != SPACE {continue;}
            if x < n-1 {queue.push_back((x+1, y));}
            if y < m-1 {queue.push_back((x, y+1));}
            if x > 0 {n_paths[x][y] += n_paths[x-1][y];}
            if y > 0 {n_paths[x][y] += n_paths[x][y-1];}
        }


        n_paths[n_paths.len()-1][n_paths[0].len()-1]
    }
}
