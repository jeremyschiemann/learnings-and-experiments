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

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                self.next = node.next.as_deref();
                Some(&node.value)
            }
            None => None,
        }
    }
}

pub struct IntoNodeIter<T>(LinkedList<T>);

impl<T> Iterator for IntoNodeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoNodeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoNodeIter(self)
    }
}
