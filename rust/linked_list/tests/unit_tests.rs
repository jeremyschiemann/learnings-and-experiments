use linked_list::LinkedList;

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
