use super::node::Node;
use std::collections::VecDeque;

pub struct InorderTraversal<'a, T: 'a + Ord> {
    pub stack: Vec<Option<&'a Box<Node<T>>>>,
    pub current: Option<&'a Box<Node<T>>>,
}

pub struct ReverseOrderTraversal<'a, T: 'a + Ord> {
    pub stack: Vec<Option<&'a Box<Node<T>>>>,
    pub current: Option<&'a Box<Node<T>>>,
}

pub struct PreorderTraversal<'a, T: 'a + Ord> {
    pub stack: Vec<Option<&'a Box<Node<T>>>>,
    pub current: Option<&'a Box<Node<T>>>,
}

pub struct PostorderTraversal<'a, T: 'a + Ord> {
    pub stack: Vec<Option<&'a Box<Node<T>>>>,
    pub current: Option<&'a Box<Node<T>>>,
}

pub struct LevelOrderTraversal<'a, T: 'a + Ord> {
    pub deque: VecDeque<&'a Box<Node<T>>>,
}

impl<'a, T: 'a + Ord> Iterator for InorderTraversal<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            if let Some(current) = self.current {
                self.stack.push(self.current);
                self.current = current.left.0.as_ref();
            } else {
                if let Some(node) = self.stack.pop() {
                    let current = node.unwrap();
                    let element = &current.value;
                    self.current = current.right.0.as_ref();
                    return Some(element);
                } else {
                    return None;
                }
            }
        }
    }
}

impl<'a, T: 'a + Ord> Iterator for ReverseOrderTraversal<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            if let Some(current) = self.current {
                self.stack.push(self.current);
                self.current = current.right.0.as_ref();
            } else {
                if let Some(node) = self.stack.pop() {
                    let current = node.unwrap();
                    let element = &current.value;
                    self.current = current.left.0.as_ref();
                    return Some(element);
                } else {
                    return None;
                }
            }
        }
    }
}

impl<'a, T: 'a + Ord> Iterator for PreorderTraversal<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            if let Some(current) = self.current {
                let element = &current.value;
                self.current = current.left.0.as_ref();
                self.stack.push(self.current);
                return Some(element);
            } else {
                if let Some(node) = self.stack.pop() {
                    if let Some(current) = node {
                        self.current = current.right.0.as_ref();
                        if self.current.is_some() {
                            self.stack.push(self.current);
                        }
                    }
                } else {
                    return None;
                }
            }
        }
    }
}

impl<'a, T: 'a + Ord> Iterator for PostorderTraversal<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            // Go down the left branch and add nodes along with their right
            // children to the stack
            while let Some(current) = self.current {
                if current.right.0.is_some() {
                    self.stack.push(current.right.0.as_ref());
                }
                self.stack.push(self.current);
                self.current = current.left.0.as_ref();
            }

            if self.stack.len() == 0 {
                return None;
            }

            if let Some(root) = self.stack.pop().unwrap() {
                // If the popped item has a right child, and the right child
                // hasn't been processed yet, make sure the right child is
                // processed before root
                if self.stack.len() > 0
                    && root.right.0.is_some()
                    && self.stack.get(self.stack.len() - 1).unwrap().unwrap().value
                        == root.right.0.as_ref().unwrap().value
                {
                    self.stack.pop(); // Remove the right child from stack
                    self.stack.push(Some(root)); // Push the root back to stack

                    // Changes the current node so that the root's right
                    // child is viewed first
                    self.current = root.right.0.as_ref();
                } else {
                    let element = &root.value;
                    self.current = None;
                    return Some(element);
                }
            } else {
                return None; // Only empty nodes remain
            }
        }
    }
}

impl<'a, T: 'a + Ord> Iterator for LevelOrderTraversal<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(boxed_node) = self.deque.pop_front() {
            if let Some(left) = boxed_node.left.0.as_ref() {
                self.deque.push_back(left);
            }

            if let Some(right) = boxed_node.right.0.as_ref() {
                self.deque.push_back(right);
            }

            Some(&boxed_node.value)
        } else {
            return None;
        }
    }
}
