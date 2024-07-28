/*
题目描述
有一棵二叉树，每个节点由一个大写字母标识(最多26个节点）。

现有两组字母，分别表示后序遍历（左孩子->右孩子->父节点）和中序遍历（左孩子->父节点->右孩子）的结果，请你输出层序遍历的结果。

输入描述
每个输入文件一行，第一个字符串表示后序遍历结果，第二个字符串表示中序遍历结果。（每串只包含大写字母）

中间用单空格分隔。

输出描述
输出仅一行，表示层序遍历的结果，结尾换行。

用例
输入	CBEFDA CBAEDF
输出	ABDCEF
说明	
二叉树为：

     A
    /   \
  B    D
 /      /  \
C    E   F
 */
use std::collections::VecDeque;

struct TreeNode {
    value: char,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: char) -> Self {
        return TreeNode {
            value,
            left: None,
            right: None,
        };
    }

    // post_order 左右根；in_order 左根右 pre_order 根左右， 层：根-》左层--》右层
    fn build_tree(postorder: &str, inorder: &str) -> Option<Box<TreeNode>> {
        if postorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let root_value = postorder.chars().last().unwrap();
        let root_index = inorder.find(root_value).unwrap();

        let left_inorder = &inorder[..root_index];
        let right_inorder = &inorder[root_index + 1..];

        let left_postorder = &postorder[..left_inorder.len()];
        let right_postorder = &postorder[left_inorder.len()..postorder.len() - 1];

        Some(Box::new(TreeNode {
            value: root_value,
            left: Self::build_tree(left_postorder, left_inorder),
            right: Self::build_tree(right_postorder, right_inorder),
        }))
    }

    fn level_order(&self) -> String {
        let mut deque = VecDeque::new();
        deque.push_back(self);
        let mut result = String::new();
        while let Some(tree) = deque.pop_front() {
            result.push(tree.value);
            if let Some(left) = &tree.left {
                deque.push_back(left);
            }
            if let Some(right) = &tree.right {
                deque.push_back(right);
            }
        }
        result
    }
}

#[cfg(test)]
pub mod tests {
    use crate::binary_tree_traversal::TreeNode;

    #[test]
    fn test() {
        let result = TreeNode::build_tree("CBEFDA", "CBAEDF");
        let result = if let Some(tree) = result {
            tree.level_order()
        } else {
            String::new()
        };
        println!("{result:?}")
    }
}