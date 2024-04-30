/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM DONE
/*
解题思路：
二叉堆是一个完全二叉树，通常实现为数组。
对于最小堆，每个节点的值都小于或等于其子节点的值。
对于最大堆，每个节点的值都大于或等于其子节点的值。
通常，堆是按照层次顺序存储在数组中的。
根据这些性质，有以下完成题目的思路。

添加元素：将新元素添加到数组的末尾，然后根据堆的性质向上调整堆，以确保它满足堆的性质。
移除顶部元素：我们将数组的第一个元素（根节点）移除，并将最后一个元素放到根的位置，然后根据堆的性质向下调整堆。
查找顶部元素：顶部元素通常就是数组的第一个元素。
 */

use std::cmp::Ord;
use std::default::Default;

/// 二叉堆数据结构
pub struct Heap<T>
    where
        T: Default,
{
    count: usize,                       // 堆中元素的数量
items: Vec<T>,                      // 堆中的元素存储在这个向量中
comparator: fn(&T, &T) -> bool,     // 比较函数，用于确定堆是最小堆还是最大堆
}

impl<T> Heap<T>
    where
        T: Default,
{
    /// 创建一个新的二叉堆
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],   // 使用默认值初始化堆，保持根节点为空
            comparator,
        }
    }

    /// 返回堆中元素的数量
    pub fn len(&self) -> usize {
        self.count
    }

    /// 检查堆是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加一个新元素
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);          // 将新元素添加到数组末尾
        self.heapify_up(self.count);     // 调整堆，确保新元素处于合适的位置
    }

    /// 在索引处向上调整堆
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if !(self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                self.items.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    /// 在索引处向下调整堆
    fn heapify_down(&mut self, idx: usize) {
        let mut idx = idx;

        loop {
            // 计算左子节点索引
            let left_child_idx = self.left_child_idx(idx);
            // 计算右子节点索引
            let right_child_idx = self.right_child_idx(idx);
            // 记录最小/最大子节点的索引，默认为左子节点
            let mut smallest_child_idx = left_child_idx;

            // 如果右子节点存在并且比左子节点小/大，则更新最小/最大子节点的索引
            if right_child_idx <= self.count && !(self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
                smallest_child_idx = right_child_idx;
            }

            // 如果当前节点比最小/最大子节点小/大，则交换它们，并更新当前索引
            if smallest_child_idx <= self.count && !(self.comparator)(&self.items[idx], &self.items[smallest_child_idx]) {
                self.items.swap(idx, smallest_child_idx);
                idx = smallest_child_idx;
            } else {
                break; // 如果当前节点已经比子节点小/大，则堆已调整完成，退出循环
            }
        }
    }

    /// 返回指定索引的父节点索引
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    /// 检查索引处是否有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /// 返回指定索引的左子节点索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    /// 返回指定索引的右子节点索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 返回指定索引的较小子节点索引（对于最小堆）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 如果右子节点存在且其值比左子节点小，则返回右子节点索引，否则返回左子节点索引
        if right_idx <= self.count && (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
            right_idx
        } else {
            left_idx
        }
    }
}

impl<T> Heap<T>
    where
        T: Default + Ord,
{
    /// 创建一个新的最小堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建一个新的最大堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

/// 迭代器，用于迭代堆中的元素
impl<T> Iterator for Heap<T>
    where
        T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut last_item = self.items.pop().unwrap(); // 移除最后一个元素

        let top = self.items.swap_remove(1); // 移除根节点（堆顶元素）
        self.count -= 1; // 更新堆的元素数量

        // 如果还有元素，将最后一个元素放到根的位置，并向下调整堆
        if self.count > 0 {
            self.items.insert(1, last_item); // 将最后一个元素放到根的位置
            self.heapify_down(1);
        }

        Some(top) // 返回移除的根节点（堆顶元素）
    }

}

/// 最小堆结构体
pub struct MinHeap;

impl MinHeap {
    /// 创建一个新的最小堆
    pub fn new<T>() -> Heap<T>
        where
            T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

/// 最大堆结构体
pub struct MaxHeap;

impl MaxHeap {
    /// 创建一个新的最大堆
    pub fn new<T>() -> Heap<T>
        where
            T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}