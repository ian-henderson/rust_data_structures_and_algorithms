use super::node::Node;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Tree<T: Ord>(pub Option<Box<Node<T>>>);

impl<T: Ord> Tree<T> {
    pub fn insert(&mut self, value: T, allow_duplicate: bool) -> bool {
        let mut current = self;
        let mut is_duplicate = false;

        while let Some(ref mut node) = current.0 {
            match node.value.cmp(&value) {
                Ordering::Greater => current = &mut node.left,
                Ordering::Less => current = &mut node.right,
                Ordering::Equal => {
                    if allow_duplicate {
                        is_duplicate = true;
                        current = &mut node.right;
                    } else {
                        return true;
                    }
                }
            };
        }

        current.0 = Some(Box::new(Node::new(value)));

        is_duplicate
    }

    pub fn contains(&self, target: &T) -> bool {
        let mut current = self;

        while let Some(ref node) = current.0 {
            match node.value.cmp(target) {
                Ordering::Greater => current = &node.left,
                Ordering::Less => current = &node.right,
                Ordering::Equal => return true,
            }
        }

        false
    }

    pub fn min(&self) -> Option<&T> {
        if self.0.is_some() {
            let mut current = self.0.as_ref();
            let mut left = current.unwrap().left.0.as_ref();

            while let Some(node) = left {
                current = left;
                left = node.left.0.as_ref();
            }

            current.map(|node| &node.value)
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<&T> {
        if self.0.is_some() {
            let mut current = self.0.as_ref();
            let mut right = current.unwrap().right.0.as_ref();

            while let Some(node) = right {
                current = right;
                right = node.right.0.as_ref();
            }

            current.map(|node| &node.value)
        } else {
            None
        }
    }

    pub fn successor(&self, value: &T) -> Option<&T> {
        let mut current = self.0.as_ref();
        let mut successor = None;

        while let Some(node) = current {
            if node.value > *value {
                successor = current;
                current = node.left.0.as_ref();
            } else {
                current = node.right.0.as_ref();
            }
        }

        successor.map(|node| &node.value)
    }

    pub fn predecessor(&self, value: &T) -> Option<&T> {
        let mut current = self.0.as_ref();
        let mut predecessor = None;

        while let Some(node) = current {
            if node.value < *value {
                predecessor = current;
                current = node.right.0.as_ref();
            } else {
                current = node.left.0.as_ref();
            }
        }

        predecessor.map(|node| &node.value)
    }

    pub fn extract_min(&mut self) -> Option<T> {
        let mut to_return = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().left.0.is_some() {
                current = &mut current.0.as_mut().unwrap().left;
            }

            let node = current.0.take().unwrap();
            to_return = Some(node.value);
            current.0 = node.right.0;
        }

        to_return
    }

    pub fn extract_max(&mut self) -> Option<T> {
        let mut to_return = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().right.0.is_some() {
                current = &mut current.0.as_mut().unwrap().right;
            }

            let node = current.0.take().unwrap();
            to_return = Some(node.value);
            current.0 = node.left.0;
        }

        to_return
    }

    pub fn remove(&mut self, target: &T) -> bool {
        let mut current: *mut Tree<T> = self;

        unsafe {
            while let Some(ref mut node) = (*current).0 {
                match node.value.cmp(target) {
                    Ordering::Greater => current = &mut node.left,
                    Ordering::Less => current = &mut node.right,
                    Ordering::Equal => {
                        match (node.left.0.as_mut(), node.right.0.as_mut()) {
                            // Node has no children
                            (None, None) => (*current).0 = None,
                            // Replace node with its left child
                            (Some(_), None) => (*current).0 = node.left.0.take(),
                            // Replace node with its right child
                            (None, Some(_)) => (*current).0 = node.right.0.take(),
                            // Replace vaue of current node with its successor,
                            // then remove the successor's node
                            (Some(_), Some(_)) => {
                                (*current).0.as_mut().unwrap().value =
                                    node.right.extract_min().unwrap()
                            }
                        }

                        return true; // removal occurred
                    }
                }
            }
        }

        false // an element with 'target' value was not found
    }

    pub fn sorted_vec(&self) -> Vec<&T> {
        let mut elements = Vec::new();

        if let Some(node) = self.0.as_ref() {
            elements.extend(node.left.sorted_vec());
            elements.push(&node.value);
            elements.extend(node.right.sorted_vec());
        }

        elements
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        let mut elements = Vec::new();

        if let Some(node) = self.0 {
            elements.extend(node.left.into_sorted_vec());
            elements.push(node.value);
            elements.extend(node.right.into_sorted_vec());
        }

        elements
    }
}
