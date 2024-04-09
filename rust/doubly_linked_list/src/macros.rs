#[macro_export]
macro_rules! linked_list {
    ($elem:expr; $n:expr) => {{
        let mut list = DoubleLinkedList::new();
        for _ in 0..$n {
            list.append($elem.clone());
        }
        list
    }};
    ($($elem:expr),*) => {{
        let mut list = DoubleLinkedList::new();
        $(
            list.append($elem);
        )*
        list
    }};
}
