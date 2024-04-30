/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM  DONE
use std::cmp::Ordering;
use std::fmt::Debug;

// 定义树节点
#[derive(Debug)]
struct TreeNode<T>
    where
        T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// 定义二叉搜索树
#[derive(Debug)]
struct BinarySearchTree<T>
    where
        T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
    where
        T: Ord,
{
    // 创建新的树节点
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
    where
        T: Ord,
{
    // 创建新的二叉搜索树
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入值到二叉搜索树
    fn insert(&mut self, value: T) {
        if let Some(ref mut node) = self.root {
            node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // 在二叉搜索树中搜索值
    fn search(&self, value: T) -> bool {
        if let Some(ref node) = self.root {
            node.search(value)
        } else {
            false
        }
    }
}

impl<T> TreeNode<T>
    where
        T: Ord,
{
    // 向树节点插入值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 对于重复值，我们这里不做处理，也可以选择其他方式处理
            }
        }
    }

    // 在树节点中搜索值
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref left) = self.left {
                    left.search(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right) = self.right {
                    right.search(value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 空树搜索任何值都应该返回false
        assert_eq!(bst.search(1), false);

        // 插入一些值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 检查插入的值是否存在
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 检查不存在的值
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值
        bst.insert(1);
        bst.insert(1);

        // 检查重复值是否存在
        assert_eq!(bst.search(1), true);

        // 检查重复值节点是否正确处理
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}



