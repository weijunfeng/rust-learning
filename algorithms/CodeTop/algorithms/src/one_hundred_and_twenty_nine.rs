/*
https://leetcode.cn/problems/sum-root-to-leaf-numbers/description/
129. 求根节点到叶节点数字之和
给你一个二叉树的根节点 root ，树中每个节点都存放有一个 0 到 9 之间的数字。
每条从根节点到叶节点的路径都代表一个数字：

例如，从根节点到叶节点的路径 1 -> 2 -> 3 表示数字 123 。
计算从根节点到叶节点生成的 所有数字之和 。

叶节点 是指没有子节点的节点。

 

示例 1：


输入：root = [1,2,3]
输出：25
解释：
从根到叶子节点路径 1->2 代表数字 12
从根到叶子节点路径 1->3 代表数字 13
因此，数字总和 = 12 + 13 = 25
示例 2：


输入：root = [4,9,0,5,1]
输出：1026
解释：
从根到叶子节点路径 4->9->5 代表数字 495
从根到叶子节点路径 4->9->1 代表数字 491
从根到叶子节点路径 4->0 代表数字 40
因此，数字总和 = 495 + 491 + 40 = 1026
 

提示：

树中节点的数目在范围 [1, 1000] 内
0 <= Node.val <= 9
树的深度不超过 10
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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut sum = 0;
        Self::recursion(&root, 0, &mut sum);
        sum
    }

    pub fn recursion(root: &Option<Rc<RefCell<TreeNode>>>, data: i32, sum: &mut i32) {
        let tree_node = root.as_ref().unwrap().borrow();
        let curr_sum = data * 10 + tree_node.val;
        if tree_node.left.is_none() && tree_node.right.is_none() {
            *sum = *sum + curr_sum;
            return;
        }
        if tree_node.left.is_some() {
            Self::recursion(&tree_node.left, curr_sum, sum);
        }
        if tree_node.right.is_some() {
            Self::recursion(&tree_node.right, curr_sum, sum);
        }
    }

    pub fn sum_numbers2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, data: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let tree_node = root.as_ref().unwrap().borrow();
        let curr_sum = data * 10 + tree_node.val;
        // 叶子节点，返回累计和
        if tree_node.left.is_none() && tree_node.right.is_none() {
            return curr_sum;
        }
        // 计算左右节点的数据和
        let left_sum = Self::dfs(&tree_node.left, curr_sum);
        let right_sum = Self::dfs(&tree_node.right, curr_sum);
        left_sum + right_sum
    }
}