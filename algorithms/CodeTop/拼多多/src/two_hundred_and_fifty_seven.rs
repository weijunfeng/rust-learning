/*
https://leetcode.cn/problems/binary-tree-paths/description/
257. 二叉树的所有路径
给你一个二叉树的根节点 root ，按 任意顺序 ，返回所有从根节点到叶子节点的路径。

叶子节点 是指没有子节点的节点。

 
示例 1：


输入：root = [1,2,3,null,5]
输出：["1->2->5","1->3"]
示例 2：

输入：root = [1]
输出：["1"]
 

提示：

树中节点的数目在范围 [1, 100] 内
-100 <= Node.val <= 100
 */

use std::cell::RefCell;
use std::rc::Rc;

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
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        Solution::dfs(&root, "", &mut result);
        result
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, path: &str, paths: &mut Vec<String>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        let mut path = path.to_string();
        path.push_str(&node.val.to_string());
        if node.left == None && node.right == None {
            paths.push(path.to_string());
            return;
        }
        path.push_str("=>");
        Solution::dfs(&node.left, &path, paths);
        Solution::dfs(&node.right, &path, paths);
    }
}