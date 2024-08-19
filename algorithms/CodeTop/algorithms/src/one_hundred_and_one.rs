/*
https://leetcode.cn/problems/symmetric-tree/description/
101. 对称二叉树
给你一个二叉树的根节点 root ， 检查它是否轴对称。

 

示例 1：


输入：root = [1,2,2,3,4,4,3]
输出：true
示例 2：


输入：root = [1,2,2,null,3,null,3]
输出：false
 

提示：

树中节点数目在范围 [1, 1000] 内
-100 <= Node.val <= 100
 

进阶：你可以运用递归和迭代两种方法解决这个问题吗？
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
    // 对称树，左右子树应该是最左边等于最右边&&最右边等于最左边
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root_node = root.as_ref().unwrap().borrow();
        Self::is_mirror(&root_node.left, &root_node.right)
    }

    // 1. 根节点值相同，2. 左子树的左子树和右子树的右子树是对称的，3. 左子树的右子树和右子树的左子树是对称的
    pub fn is_mirror(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }
        let left_node = left.as_ref().unwrap().borrow();
        let right_node = right.as_ref().unwrap().borrow();
        if left_node.val != right_node.val {
            return false;
        }
        Self::is_mirror(&left_node.left, &right_node.right) && Self::is_mirror(&left_node.right, &right_node.left)
    }

    pub fn is_mirror2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let ref root_node = root.as_ref().unwrap().borrow();
        let mut queue = vec![];
        queue.push(root_node.left.clone());
        queue.push(root_node.right.clone());
        while !queue.is_empty() {
            let right = queue.pop().unwrap();
            let left = queue.pop().unwrap();
            if left.is_none() && right.is_none() {
                continue;
            }
            if left.is_none() || right.is_none() {
                return false;
            }
            let left_node = left.as_ref().unwrap().borrow();
            let right_node = right.as_ref().unwrap().borrow();
            if left_node.val != right_node.val {
                return false;
            }
            queue.push(left_node.left.clone());
            queue.push(right_node.right.clone());

            queue.push(right_node.left.clone());
            queue.push(left_node.right.clone());
        }
        true
    }
}