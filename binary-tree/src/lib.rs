//#![feature(nll)]

use std::fmt::Debug;
use std::mem::swap;

#[allow(dead_code)]
#[derive(Debug)]
struct BinaryTree<T>
where
    T: Ord + Debug,
{
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node<T>
where
    T: Ord + Debug,
{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T> Node<T>
where
    T: Ord + Debug,
{
    fn new_box(value: T) -> Box<Node<T>> {
        Box::new(Node::<T> {
            value,
            left: None,
            right: None,
        })
    }
}

#[allow(dead_code)]
impl<T> BinaryTree<T>
where
    T: Ord + Debug,
{
    fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    fn append(&mut self, value: T) {
        let mut current_link: &mut Option<Box<Node<T>>> = &mut self.root;

        while let Some(ref mut current_box) = current_link {
            if value < current_box.value {
                current_link = &mut current_box.left;
            } else {
                current_link = &mut current_box.right;
            }
        }

        *current_link = Some(Node::new_box(value));
    }

    fn reverse(&mut self) {
        return BinaryTree::reverse_internal(&mut self.root);
    }

    fn reverse_internal(root_link: &mut Option<Box<Node<T>>>) {
        if let Some(ref mut root_box) = root_link {
            swap(&mut root_box.left, &mut root_box.right);
            BinaryTree::reverse_internal(&mut root_box.left);
            BinaryTree::reverse_internal(&mut root_box.right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = BinaryTree::<i32>::new();
        assert_eq!(tree.root.is_none(), true);
    }

    #[test]
    fn test_append() {
        let mut tree = BinaryTree::<i32>::new();
        tree.append(6);
        tree.append(3);
        tree.append(9);

        tree.append(1);
        tree.append(5);

        tree.append(7);
        tree.append(11);

        let root: &Box<Node<i32>> = tree.root.as_ref().unwrap();
        let left_subtree: &Box<Node<i32>> = root.left.as_ref().unwrap();
        let right_subtree: &Box<Node<i32>> = root.right.as_ref().unwrap();
        // Checking leaves.
        assert_eq!(left_subtree.left.as_ref().unwrap().left.is_none(), true);
        assert_eq!(left_subtree.left.as_ref().unwrap().right.is_none(), true);
        assert_eq!(left_subtree.left.as_ref().unwrap().value, 1);

        assert_eq!(left_subtree.right.as_ref().unwrap().left.is_none(), true);
        assert_eq!(left_subtree.right.as_ref().unwrap().right.is_none(), true);
        assert_eq!(left_subtree.right.as_ref().unwrap().value, 5);

        assert_eq!(right_subtree.left.as_ref().unwrap().left.is_none(), true);
        assert_eq!(right_subtree.left.as_ref().unwrap().right.is_none(), true);
        assert_eq!(right_subtree.left.as_ref().unwrap().value, 7);

        assert_eq!(right_subtree.right.as_ref().unwrap().left.is_none(), true);
        assert_eq!(right_subtree.right.as_ref().unwrap().right.is_none(), true);
        assert_eq!(right_subtree.right.as_ref().unwrap().value, 11);

        // Paths to leaves already checked implicitly.

        // Checking non-leaf node values.
        assert_eq!(left_subtree.value, 3);
        assert_eq!(right_subtree.value, 9);
        assert_eq!(root.value, 6);
    }

    #[test]
    fn test_reverse() {
        let mut tree = BinaryTree::<i32>::new();
        tree.append(6);
        tree.append(3);
        tree.append(9);

        tree.append(1);
        tree.append(5);

        tree.append(7);
        tree.append(11);

        tree.reverse();

        let root: &Box<Node<i32>> = tree.root.as_ref().unwrap();
        let left_subtree: &Box<Node<i32>> = root.left.as_ref().unwrap();
        let right_subtree: &Box<Node<i32>> = root.right.as_ref().unwrap();
        // Checking leaves.
        assert_eq!(left_subtree.left.as_ref().unwrap().left.is_none(), true);
        assert_eq!(left_subtree.left.as_ref().unwrap().right.is_none(), true);
        assert_eq!(left_subtree.left.as_ref().unwrap().value, 11);

        assert_eq!(left_subtree.right.as_ref().unwrap().left.is_none(), true);
        assert_eq!(left_subtree.right.as_ref().unwrap().right.is_none(), true);
        assert_eq!(left_subtree.right.as_ref().unwrap().value, 7);

        assert_eq!(right_subtree.left.as_ref().unwrap().left.is_none(), true);
        assert_eq!(right_subtree.left.as_ref().unwrap().right.is_none(), true);
        assert_eq!(right_subtree.left.as_ref().unwrap().value, 5);

        assert_eq!(right_subtree.right.as_ref().unwrap().left.is_none(), true);
        assert_eq!(right_subtree.right.as_ref().unwrap().right.is_none(), true);
        assert_eq!(right_subtree.right.as_ref().unwrap().value, 1);

        // Paths to leaves already checked implicitly.

        // Checking non-leaf node values.
        assert_eq!(left_subtree.value, 9);
        assert_eq!(right_subtree.value, 3);
        assert_eq!(root.value, 6);
    }
}
