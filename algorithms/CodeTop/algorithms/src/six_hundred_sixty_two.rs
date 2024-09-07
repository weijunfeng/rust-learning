/*
https://leetcode.cn/problems/maximum-width-of-binary-tree/description/
662. 二叉树最大宽度
给你一棵二叉树的根节点 root ，返回树的 最大宽度 。

树的 最大宽度 是所有层中最大的 宽度 。

每一层的 宽度 被定义为该层最左和最右的非空节点（即，两个端点）之间的长度。将这个二叉树视作与满二叉树结构相同，两端点间会出现一些延伸到这一层的 null 节点，这些 null 节点也计入长度。

题目数据保证答案将会在  32 位 带符号整数范围内。



示例 1：


输入：root = [1,3,2,5,3,null,9]
输出：4
解释：最大宽度出现在树的第 3 层，宽度为 4 (5,3,null,9) 。
示例 2：


输入：root = [1,3,2,5,null,null,9,6,null,7]
输出：7
解释：最大宽度出现在树的第 4 层，宽度为 7 (6,null,null,null,null,null,7) 。
示例 3：


输入：root = [1,3,2,5]
输出：2
解释：最大宽度出现在树的第 2 层，宽度为 2 (3,2) 。


提示：

树中节点的数目范围是 [1, 3000]
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
    // 使用广度优先搜索（BFS），给每个节点一个编号来表示它在该层的位置，计算每一层最右边和最左边的编号差值来得到最大宽度
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut max_length = 0;
        let mut deque = VecDeque::new();
        // 将根节点和位置编号 0加入队列
        deque.push_back((root.unwrap(), 0));
        while !deque.is_empty() {
            let size = deque.len();
            // 当前层最左边节点的位置
            let min_index = deque.front().unwrap().1;
            let mut first = 0;
            let mut last = 0;
            for i in 0..size {
                let (node, index) = deque.pop_front().unwrap();
                let index = index - min_index; // 防止位置数溢出
                if i == 0 { first = index; } // 当前层的第一个节点
                if i == size - 1 { last = index; } // 当前层的最后一个节点
                let node_ref = node.borrow();
                if let Some(left) = node_ref.left.clone() {
                    deque.push_back((left, 2 * index));
                }
                if let Some(right) = node_ref.right.clone() {
                    deque.push_back((right, 2 * index + 1));
                }
            }
            max_length = max_length.max(last - first + 1); // 更新最大宽度
        }
        max_length
    }
}