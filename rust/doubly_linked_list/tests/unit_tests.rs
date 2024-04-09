use doubly_linked_list::prelude::*;

#[test]
fn new_is_zero() {
    let list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    assert_eq!(list.len(), 0);
}

#[test]
fn append_first() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.append(10);
    assert_eq!(list.len(), 1);
}

#[test]
fn append_multiple() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    assert_eq!(list.len(), 3);
}

#[test]
fn push_first() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.push(10);
    assert_eq!(list.len(), 1);
}

#[test]
fn push_multiple() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.len(), 3);
}

#[test]
fn append_pop_order() {
    let mut list: DoubleLinkedList<usize> = DoubleLinkedList::new();

    for n in 1..=3_usize {
        list.append(n);
    }

    for n in 3..=1_usize {
        assert_eq!(list.pop_tail(), Some(n));
    }
}

#[test]
fn append_pop_order2() {
    let mut list: DoubleLinkedList<usize> = DoubleLinkedList::new();

    for n in 1..=3_usize {
        list.append(n);
    }

    for n in 1..=3_usize {
        assert_eq!(list.pop_head(), Some(n));
    }
}

#[test]
fn push_pop_order() {
    let mut list: DoubleLinkedList<usize> = DoubleLinkedList::new();

    for n in 1..=3_usize {
        list.push(n);
    }

    for n in 1..=3_usize {
        assert_eq!(list.pop_tail(), Some(n));
    }
}

#[test]
fn push_pop_order2() {
    let mut list: DoubleLinkedList<usize> = DoubleLinkedList::new();

    for n in 1..=3_usize {
        list.push(n);
    }

    for n in 3..=1_usize {
        assert_eq!(list.pop_tail(), Some(n));
    }
}

#[test]
fn linkedlist_from_vec() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let list: DoubleLinkedList<i32> = my_vec.into();
    assert_eq!(list.len(), 5)
}

#[test]
fn vec_from_linkedlist() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();

    for n in 1..=5 {
        list.append(n);
    }

    let my_vec: Vec<i32> = list.into();
    assert_eq!(my_vec.len(), 5)
}

#[cfg(feature = "macros")]
#[test]
fn new_from_macros() {
    let mut list = linked_list![1, 2, 3, 4, 5];
    assert_eq!(list.len(), 5);
    for n in list.len()..=1 {
        assert_eq!(list.pop_tail(), Some(n));
    }

    let mut list = linked_list![0; 5];
    assert_eq!(list.len(), 5);
    while let Some(n) = list.pop_tail() {
        assert_eq!(n, 0);
    }
}
