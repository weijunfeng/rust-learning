/*
https://leetcode.cn/problems/binary-tree-preorder-traversal/description/
144. 二叉树的前序遍历
给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

 

示例 1：


输入：root = [1,null,2,3]
输出：[1,2,3]
示例 2：

输入：root = []
输出：[]
示例 3：

输入：root = [1]
输出：[1]
示例 4：


输入：root = [1,2]
输出：[1,2]
示例 5：


输入：root = [1,null,2]
输出：[1,2]
 

提示：

树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100
 

进阶：递归算法很简单，你可以通过迭代算法完成吗？
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
    // 前序遍历：根左右
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut value = vec![];
        Self::recursion(&root, &mut value);
        value
    }

    pub fn recursion(root: &Option<Rc<RefCell<TreeNode>>>, value: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root_node = root.as_ref().unwrap().borrow();
        value.push(root_node.val);
        Self::recursion(&root_node.left, value);
        Self::recursion(&root_node.right, value);
    }
}