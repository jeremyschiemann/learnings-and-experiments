use crate::{LinkedList, Node};

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

impl<'a, T> LinkedList<T> {
    // Method to create a non-consuming iterator
    pub fn iter(&'a self) -> NodeIter<'a, T> {
        NodeIter {
            next: self.head.as_deref(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_tail(10);
        list.push_tail(20);
        list.push_tail(5);

        let c: i32 = list.iter().map(|x| x * 2).sum();
        assert_eq!(c, 70);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn into_iterator() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_tail(10);
        list.push_tail(20);
        list.push_tail(5);

        let c: i32 = list.into_iter().map(|x| x * 2).sum();
        assert_eq!(c, 70);
    }
}

