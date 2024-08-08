/*
https://leetcode.cn/problems/binary-tree-maximum-path-sum/description/
124. 二叉树中的最大路径和
二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。

 

示例 1：


输入：root = [1,2,3]
输出：6
解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
示例 2：


输入：root = [-10,9,20,null,null,15,7]
输出：42
解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42
 

提示：

树中节点数目范围是 [1, 3 * 104]
-1000 <= Node.val <= 1000
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = i32::MIN;
        Solution::get_max(&root, &mut sum);
        sum
    }

    pub fn get_max(head: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(n) = head {
            let left = Solution::get_max(&n.borrow().left, max).max(0);
            let right = Solution::get_max(&n.borrow().right, max).max(0);
            let head = n.borrow().val;
            let curr_sum = left + right + head;
            *max = (*max).max(curr_sum);
            return head + left.max(right);
        }
        0
    }

    pub fn max_path_sum1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = i32::MIN;
        Solution::get_max1(&root, &mut ret);
        ret
    }

    fn get_max1(node: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(n) = node {
            let left = Solution::get_max1(&n.borrow().left, max).max(0);  // 左子树的最大贡献值
            let right = Solution::get_max1(&n.borrow().right, max).max(0); // 右子树的最大贡献值
            *max = (*max).max(n.borrow().val + left + right); // 更新全局最大路径和
            return n.borrow().val + left.max(right); // 返回当前节点的最大贡献值
        }
        0
    }
}