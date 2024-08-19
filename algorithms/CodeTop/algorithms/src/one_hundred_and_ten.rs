/*
https://leetcode.cn/problems/balanced-binary-tree/description/
110. 平衡二叉树
给定一个二叉树，判断它是否是 平衡二叉树
  
平衡二叉树 是指该树所有节点的左右子树的深度相差不超过 1。
 

示例 1：


输入：root = [3,9,20,null,null,15,7]
输出：true
示例 2：


输入：root = [1,2,2,3,3,null,null,4,4]
输出：false
示例 3：

输入：root = []
输出：true
 

提示：

树中的节点数在范围 [0, 5000] 内
-104 <= Node.val <= 104
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::height(&root) != -1
    }

    pub fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left_height = Self::height(&root.as_ref().unwrap().borrow().left);
        let right_height = Self::height(&root.as_ref().unwrap().borrow().right);
        // 左子树或右子树高度为-1 或高度差大于 1，则返回-1，标识不为平衡二叉数
        if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
            return -1;
        }
        left_height.max(right_height) + 1
    }
}