#![allow(unused)]

use std::fmt::Display;

struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy + Display + PartialEq> SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    fn head(&self) -> Option<T> {
        if let Some(node) = &self.head {
            return Some(node.value);
        }
        None
    }

    fn len(&self) -> u32 {
        let mut len = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }
        len
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            return Some(head.value);
        }
        None
    }

    fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }
        println!("None");
    }

    fn push(&mut self, value: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { value, next: None }));
    }

    fn tail(&self) -> Option<T> {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            if node.next.is_none() {
                return Some(node.value);
            }
            current = node.next.as_ref();
        }
        None
    }

    fn to_vector(&self) -> Vec<T> {
        let mut v = vec![];
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            v.push(node.value);
            current = node.next.as_ref();
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_with_i32() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.to_vector(), vec![1, 2, 3]);
    }

    #[test]
    fn should_work_with_str() {
        let mut list = SinglyLinkedList::new();
        list.push("one");
        list.push("two");
        list.push("three");
        assert_eq!(list.to_vector(), vec!["one", "two", "three"]);
    }

    #[test]
    fn should_calculate_len() {
        let mut list = SinglyLinkedList::new();
        list.push("one");
        list.push("two");
        list.push("three");
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn should_pop_items() {
        let mut list = SinglyLinkedList::new();
        list.push("one");
        list.push("two");
        list.push("three");
        list.push("four");
        let value = list.pop();
        assert_eq!(list.to_vector(), vec!["two", "three", "four"]);
        assert_eq!(value.unwrap(), "one");
    }

    #[test]
    fn should_return_head() {
        let mut list = SinglyLinkedList::new();
        list.push("one");
        list.push("two");
        list.push("three");
        list.push("four");
        let head = list.head();
        assert_eq!(head.unwrap(), "one");
    }

    #[test]
    fn should_return_tail() {
        let mut list = SinglyLinkedList::new();
        list.push("one");
        list.push("two");
        list.push("three");
        list.push("four");
        let tail = list.tail();
        assert_eq!(tail.unwrap(), "four");
    }
}
