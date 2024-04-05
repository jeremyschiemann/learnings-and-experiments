use doubly_linked_list::DoubleLinkedList;

#[test]
fn new() {
    let list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    assert!(list.head.is_none());
}

#[test]
fn push() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.push_head(40);
    list.push_head(50);
    assert!(list.head.is_some());
}

#[test]
fn pop_none() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    assert_eq!(list.pop_tail(), None);
}

#[test]
fn pop() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.push_head(40);
    list.push_head(50);
    assert_eq!(list.pop_tail(), Some(50));
    assert_eq!(list.pop_tail(), Some(40));
    assert_eq!(list.pop_tail(), None);
}
