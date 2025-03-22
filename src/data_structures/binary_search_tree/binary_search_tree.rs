use super::{
    traversal::{
        InorderTraversal, LevelOrderTraversal, PostorderTraversal, PreorderTraversal,
        ReverseOrderTraversal,
    },
    tree::Tree,
};
use std::{
    cmp::PartialEq,
    collections::VecDeque,
    fmt,
    iter::{Extend, FromIterator},
};

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: Tree<T>,
    pub size: usize,
}

impl<T: Ord> PartialEq for BinarySearchTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_vec() == other.sorted_vec()
    }
}

impl<T: Ord + fmt::Debug> fmt::Display for BinarySearchTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.sorted_vec())
    }
}

impl<T: Ord> Extend<T> for BinarySearchTree<T> {
    /// Extends BinarySearchTree elements from iterators
    /// # Example:
    /// ```
    /// use data_structures_and_algorithms::data_structures::binary_search_tree::BinarySearchTree;
    /// use std::iter::Extend;
    ///
    /// let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    /// tree.extend(vec![7, 1, 0, 4, 5, 3].into_iter());
    /// assert_eq!(tree.sorted_vec(), [&0, &1, &3, &4, &5, &7]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |element| {
            self.insert(element);
        });
    }
}

impl<T: Ord> FromIterator<T> for BinarySearchTree<T> {
    /// Extends BinarySearchTree with elements from the iterator.
    /// # Examples:
    /// ```
    /// use data_structures_and_algorithms::data_structures::binary_search_tree::BinarySearchTree;
    /// use std::iter::Extend;
    ///
    /// let mut tree: BinarySearchTree<i32> = BinarySearchTree::from_iter(
    ///     vec![7, 1, 0, 4, 5, 3].into_iter());
    /// assert_eq!(tree.sorted_vec(), [&0, &1, &3, &4, &5, &7]);
    ///
    /// let tree: BinarySearchTree<i32> = vec![7, 1, 0, 4, 5, 3].into_iter().collect();
    /// assert_eq!(tree.sorted_vec(), [&0, &1, &3, &4, &5, &7]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = BinarySearchTree::new();
        tree.extend(iter);
        tree
    }
}

impl<T: Ord + Clone> Clone for BinarySearchTree<T> {
    fn clone(&self) -> Self {
        let mut new_tree = BinarySearchTree::new();

        for element in self.sorted_vec().iter() {
            new_tree.insert((*element).clone());
        }

        new_tree
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: Tree(None),
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        *self = BinarySearchTree::new();
    }

    pub fn root(&self) -> Option<&T> {
        self.root.0.as_ref().map(|node| &node.value)
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.size += 1;
        self.root.insert(value, true)
    }

    pub fn insert_without_dup(&mut self, value: T) -> bool {
        let res = self.root.insert(value, false);
        if !res {
            self.size += 1;
        }
        res
    }

    pub fn contains(&self, target: &T) -> bool {
        self.root.contains(target)
    }

    pub fn min(&self) -> Option<&T> {
        self.root.min()
    }

    pub fn max(&self) -> Option<&T> {
        self.root.max()
    }

    pub fn successor(&self, value: &T) -> Option<&T> {
        self.root.successor(value)
    }

    pub fn predecessor(&self, value: &T) -> Option<&T> {
        self.root.predecessor(value)
    }

    pub fn extract_min(&mut self) -> Option<T> {
        let res = self.root.extract_min();
        if res.is_some() {
            self.size -= 1;
        }
        res
    }

    pub fn extract_max(&mut self) -> Option<T> {
        let res = self.root.extract_max();
        if res.is_some() {
            self.size -= 1;
        }
        res
    }

    pub fn remove(&mut self, target: &T) -> bool {
        let res = self.root.remove(target);
        if res {
            self.size -= 1;
        }
        res
    }

    pub fn sorted_vec(&self) -> Vec<&T> {
        self.root.sorted_vec()
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.root.into_sorted_vec()
    }

    pub fn inorder(&self) -> InorderTraversal<T> {
        InorderTraversal {
            stack: Vec::new(),
            current: self.root.0.as_ref(),
        }
    }

    pub fn reverse_order(&self) -> ReverseOrderTraversal<T> {
        ReverseOrderTraversal {
            stack: Vec::new(),
            current: self.root.0.as_ref(),
        }
    }

    pub fn preorder(&self) -> PreorderTraversal<T> {
        PreorderTraversal {
            stack: vec![self.root.0.as_ref()],
            current: self.root.0.as_ref(),
        }
    }

    pub fn postorder(&self) -> PostorderTraversal<T> {
        PostorderTraversal {
            stack: Vec::new(),
            current: self.root.0.as_ref(),
        }
    }

    pub fn level_order(&self) -> LevelOrderTraversal<T> {
        let mut deque = VecDeque::new();
        if let Some(root) = self.root.0.as_ref() {
            deque.push_back(root);
        }
        LevelOrderTraversal { deque }
    }
}
