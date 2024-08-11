/*
https://leetcode.cn/problems/binary-tree-right-side-view/description/
199. 二叉树的右视图
给定一个二叉树的 根节点 root，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。

 

示例 1:



输入: [1,2,3,null,5,null,4]
输出: [1,3,4]
示例 2:

输入: [1,null,3]
输出: [1,3]
示例 3:

输入: []
输出: []
 

提示:

二叉树的节点个数的范围是 [0,100]
-100 <= Node.val <= 100 
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    // 思路，使用层序遍历,取最右边的节点数，即层级的最后一个数据
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut result = Vec::new();
        let mut de = VecDeque::new();
        de.push_front(root);
        while !de.is_empty() {
            let len = de.len();
            for i in 0..len {
                if let Some(Some(node)) = de.pop_front() {
                    let node = node.borrow();
                    if i == len - 1 {
                        result.push(node.val);
                    }
                    // 先加左子树，后续如果没有右子树，则左子树即为当前层的最右侧节点
                    if node.left.is_some() {
                        de.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        de.push_back(node.right.clone());
                    }
                }
            }
        }
        result
    }
}