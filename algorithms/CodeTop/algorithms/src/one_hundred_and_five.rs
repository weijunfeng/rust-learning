/*
https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
105. 从前序与中序遍历序列构造二叉树
给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历， inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。

 

示例 1:


输入: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
输出: [3,9,20,null,null,15,7]
示例 2:

输入: preorder = [-1], inorder = [-1]
输出: [-1]
 

提示:

1 <= preorder.length <= 3000
inorder.length == preorder.length
-3000 <= preorder[i], inorder[i] <= 3000
preorder 和 inorder 均 无重复 元素
inorder 均出现在 preorder
preorder 保证 为二叉树的前序遍历序列
inorder 保证 为二叉树的中序遍历序列
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_recursive(&preorder, 0, preorder.len() - 1, &inorder, 0, inorder.len() - 1)
    }

    // 前序遍历：根左右，中序遍历：左根右
    // 前序遍历的第一个为根，中序遍历的对应数据左侧为左子树，右侧为右子树
    pub fn build_tree_recursive(preorder: &Vec<i32>, preorder_start: usize, preorder_end: usize,
                                inorder: &Vec<i32>, inorder_start: usize, inorder_end: usize)
                                -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_start > preorder_end || inorder_start > inorder_end {
            return None;
        }
        let root = preorder[preorder_start];
        // 寻找根节点在中序遍历位置
        let mut root_inorder_index = 0;
        for (index, &data) in inorder.iter().enumerate() {
            if data == root {
                root_inorder_index = index;
            }
        }
        if root_inorder_index < inorder_start {
            return None;
        }
        // 当前根节点，左子树的长度
        let left_tree_size = root_inorder_index - inorder_start;
        let mut root = TreeNode::new(root);
        if root_inorder_index >= 1 {
            // preorder_start + 1, preorder_start + left_tree_size 为下一个左子树前序遍历数据的开始结束位置
            root.left = Self::build_tree_recursive(preorder, preorder_start + 1, preorder_start + left_tree_size,
                                                   inorder, inorder_start, root_inorder_index - 1);
        }
        // preorder_start + left_tree_size + 1, preorder_end 为下一个右子树前序遍历数据的开始结束位置
        root.right = Self::build_tree_recursive(preorder, preorder_start + left_tree_size + 1, preorder_end,
                                                inorder, root_inorder_index + 1, inorder_end);
        Some(Rc::new(RefCell::new(root)))
    }
}