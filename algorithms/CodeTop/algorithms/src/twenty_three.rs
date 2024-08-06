/*
https://leetcode.cn/problems/merge-k-sorted-lists/description/

23. 合并 K 个升序链表

给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。

 

示例 1：

输入：lists = [[1,4,5],[1,3,4],[2,6]]
输出：[1,1,2,3,4,4,5,6]
解释：链表数组如下：
[
  1->4->5,
  1->3->4,
  2->6
]
将它们合并到一个有序链表中得到。
1->1->2->3->4->4->5->6
示例 2：

输入：lists = []
输出：[]
示例 3：

输入：lists = [[]]
输出：[]
 

提示：

k == lists.length
0 <= k <= 10^4
0 <= lists[i].length <= 500
-10^4 <= lists[i][j] <= 10^4
lists[i] 按 升序 排列
lists[i].length 的总和不超过 10^4
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

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val) // Reverse the order for min-heap behavior
    }
}


impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue = Vec::new();
        let mut dump = Box::new(ListNode::new(0));
        let mut curr = &mut dump;
        for list in lists {
            if let Some(node) = list {
                queue.push(node);
            }
        }
        queue.sort_by(|f1, f2| f1.val.cmp(&f2.val).reverse());
        while let Some(mut node) = queue.pop() {
            if let Some(next) = node.next.take() {
                queue.push(next);
                queue.sort_by(|f1, f2| f1.val.cmp(&f2.val).reverse());
            }
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }
        dump.next
    }
    pub fn merge_k_lists2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();

        // Push all the head nodes into the heap
        for list in lists {
            if let Some(node) = list {
                heap.push(node);
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        // Extract the smallest element from the heap and add its next node back to the heap
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
    }

    pub fn skip_next_node(mut head: Option<Box<ListNode>>) {
        let mut current = &mut head;
        if let Some(ref mut curr_node) = current {
            // 如果当前节点和下一个节点都存在
            if let Some(ref mut next_node) = curr_node.next {
                // 将当前节点的 next 指向下下个节点
                curr_node.next = next_node.next.take();
                // 移动到下一个节点
                current = &mut curr_node.next;
            }
        }
    }
}