#[cfg(feature = "macros")]
mod macros;
pub mod prelude;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

type StrongNode<T> = Rc<RefCell<Node<T>>>;
type WeakNode<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<StrongNode<T>>,
    previous: Option<WeakNode<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            previous: None,
        }
    }
}

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: Option<StrongNode<T>>,
    tail: Option<WeakNode<T>>,
    size: usize,
}

impl<T: Clone> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.tail.take() {
            Some(old_tail) => {
                if let Some(strong_tail) = old_tail.upgrade() {
                    new_node.borrow_mut().previous = Some(Rc::downgrade(&strong_tail));
                    strong_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                    self.tail = Some(Rc::downgrade(&new_node));
                } else {
                    self.tail = Some(Rc::downgrade(&new_node));
                    self.head = Some(new_node);
                }
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }

    pub fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                self.head = Some(new_node)
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
        self.size += 1;
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let tail_node = Weak::upgrade(self.tail.as_ref()?)?;
        let tail_value = tail_node.borrow().value.clone();

        if let Some(prev_weak) = tail_node.borrow().previous.clone() {
            if let Some(prev_node) = prev_weak.upgrade() {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_weak);
            } else {
                self.head = None;
                self.tail = None;
            }
        } else {
            self.head = None;
            self.tail = None;
        }

        self.size -= 1;
        Some(tail_value)
    }

    pub fn pop_head(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let head_node = Rc::clone(self.head.as_ref()?);
        let head_value = head_node.borrow().value.clone();

        if let Some(next_node) = head_node.borrow().next.clone() {
            self.head = Some(next_node);
        } else {
            self.head = None;
            self.tail = None;
        }

        Some(head_value)
    }
}

impl<T: Clone> From<Vec<T>> for DoubleLinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = Self::new();
        value.into_iter().for_each(|x| list.append(x));
        list
    }
}

impl<T: Clone> From<DoubleLinkedList<T>> for Vec<T> {
    fn from(value: DoubleLinkedList<T>) -> Self {
        let mut vec = Vec::with_capacity(value.size);
        let mut current = value.head;

        while let Some(node) = current {
            vec.push(node.borrow().value.clone());
            current = node.borrow().next.clone();
        }

        vec
    }
}

impl<T: Clone> Default for DoubleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
