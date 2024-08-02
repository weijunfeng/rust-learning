/*
https://leetcode.cn/problems/kth-smallest-element-in-a-sorted-matrix/description/
378. 有序矩阵中第 K 小的元素
给你一个 n x n 矩阵 matrix ，其中每行和每列元素均按升序排序，找到矩阵中第 k 小的元素。
请注意，它是 排序后 的第 k 小元素，而不是第 k 个 不同 的元素。

你必须找到一个内存复杂度优于 O(n2) 的解决方案。

 

示例 1：

输入：matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
输出：13
解释：矩阵中的元素为 [1,5,9,10,11,12,13,13,15]，第 8 小元素是 13
示例 2：

输入：matrix = [[-5]], k = 1
输出：-5
 

提示：

n == matrix.length
n == matrix[i].length
1 <= n <= 300
-109 <= matrix[i][j] <= 109
题目数据 保证 matrix 中的所有行和列都按 非递减顺序 排列
1 <= k <= n2
 

进阶：

你能否用一个恒定的内存(即 O(1) 内存复杂度)来解决这个问题?
你能在 O(n) 的时间复杂度下解决这个问题吗?这个方法对于面试来说可能太超前了，但是你会发现阅读这篇文章（ this paper ）很有趣。
 */

struct Solution;
impl Solution {
    // 求最大的第 k元素，就对矩阵中元素做二分查找，一直到矩阵中小于该二分值的元素数等于 k 为止
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = matrix[0][0];
        let mut right = matrix[m - 1][n - 1];
        while left <= right {
            let mid = (left + right) / 2;
            if Solution::dd(&matrix, k, mid) {
                left = mid + 1
            } else {
                right = mid - 1
            }
        }
        return left;
    }

    fn dd(matrix: &Vec<Vec<i32>>, k: i32, mid: i32) -> bool {
        let mut col:i32 = (matrix[0].len() - 1) as i32;
        let mut row = 0;
        let mut count = 0;
        while row < matrix.len() && col >= 0 {
            if matrix[row][col as usize] <= mid {
                count = count + col + 1;
                row = row + 1;
            } else {
                col = col - 1;
            }
        }
        return count < k;
    }
}