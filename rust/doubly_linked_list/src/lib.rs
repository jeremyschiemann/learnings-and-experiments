use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Weak<RefCell<Node<T>>>>,
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

pub struct DoubleLinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, value: T) {
        match self.head.take() {
            None => {
                let new_node = Rc::new(RefCell::new(Node::new(value)));
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
            Some(old_head) => {
                let new_node = Rc::new(RefCell::new(Node::new(value)));
                new_node.borrow_mut().next = Some(old_head.clone());
                old_head.borrow_mut().previous = Some(Rc::downgrade(&old_head));
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        let tail = self.tail.take();
        tail.and_then(|tail_weak| {
            tail_weak.upgrade().map(|tail_ref| {
                let prev;
                {
                    let mut tail_mut = tail_ref.borrow_mut();
                    prev = tail_mut.previous.take();
                    if let Some(prev_ref) = prev.as_ref().and_then(|prev| prev.upgrade()) {
                        prev_ref.borrow_mut().next = None;
                        todo!("fix BorrowMutError");
                    } else {
                        self.head = None;
                    }
                }
                self.tail = prev;
                Rc::try_unwrap(tail_ref).ok().unwrap().into_inner().value
            })
        })
    }
}
