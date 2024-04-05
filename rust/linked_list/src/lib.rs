mod iterators;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            value: data,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_head(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));

        match self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_tail(&mut self, value: T) {
        let mut current = &mut self.head;

        while let Some(ref mut node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node::new(value)))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head.value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}


impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_none() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_one() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(10);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn push_two() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(10);
        list.push_head(10);

        assert_eq!(list.len(), 2);
    }

    #[test]
    fn tail_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_tail(10);
        list.push_tail(20);
        list.push_head(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), Some(20))
    }
}
