use super::tree::Tree;

#[derive(Debug)]
pub struct Node<T: Ord> {
    pub value: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}
