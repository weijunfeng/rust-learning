/*
https://leetcode.cn/problems/reverse-linked-list/
206 反转链表
给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
 

示例 1：


输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]
示例 2：


输入：head = [1,2]
输出：[2,1]
示例 3：

输入：head = []
输出：[]
 

提示：

链表中节点的数目范围是 [0, 5000]
-5000 <= Node.val <= 5000
 

进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    // 迭代写法，使用双指针，快慢指针，前一个比后一移动快一步，当慢指针移动到尾部时，就反转了
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        pre
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_recursive(head, None)
    }

    // 递归当前节点下一个指向 pre，pre是 curr节点的前一个节点，这样就把节点指向的下一个节点反转了，该题就是修改当前节点的下个指向为前一个节点
    fn reverse_recursive(curr: Option<Box<ListNode>>, pre: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return match curr {
            None => {
                pre
            }
            Some(mut node) => {
                let next = node.next.take();
                node.next = pre;
                Self::reverse_recursive(next, Some(node))
            }
        };
    }
}