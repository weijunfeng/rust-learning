/*
https://leetcode.cn/problems/number-of-islands/description/
200. 岛屿数量
给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。

 

示例 1：

输入：grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
输出：1
示例 2：

输入：grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
输出：3
 

提示：

m == grid.length
n == grid[i].length
1 <= m, n <= 300
grid[i][j] 的值为 '0' 或 '1'
 */
struct Solution;
impl Solution {
    // 题解：1为陆地，0为水，岛屿为被 0 包围的 1，边界为水
    // 求解思路：寻找被 0 包围的 1，相连的 1 可作为一个大的 1，或使用其他数字标记为相连的陆地
    // 使用标记为相连陆地的解法更清晰
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut land_num = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    Self::mark(&mut grid, i as i32, j as i32);
                    land_num = land_num + 1;
                }
            }
        }
        land_num
    }
    pub fn mark(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
        if row < 0 || col < 0 {
            return;
        }
        let row_size = row as usize;
        let col_size = col as usize;
        if row_size >= grid.len() || col_size >= grid[row_size].len() || grid[row_size][col_size] != '1' {
            return;
        }
        grid[row_size][col_size] = '2';
        Self::mark(grid, row - 1, col);
        Self::mark(grid, row + 1, col);
        Self::mark(grid, row, col - 1);
        Self::mark(grid, row, col + 1);
    }
}