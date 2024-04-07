use inplace_sorting;

#[test]
fn bubble_sort_int() {
    let mut items = vec![5, 1, 3, 2, 4, 1, 2, 55, 12, 3, 11, 14, 634];
    let length = items.len();
    inplace_sorting::bubble_sort(items.as_mut_slice());
    assert_eq!(items, [1, 1, 2, 2, 3, 3, 4, 5, 11, 12, 14, 55, 634]);
    assert_eq!(length, items.len());
}

#[test]
fn bubble_sort_str() {
    let mut items = vec!["a", "z", "t", "b"];
    let length = items.len();
    inplace_sorting::bubble_sort(items.as_mut_slice());
    assert_eq!(items, ["a", "b", "t", "z"]);
    assert_eq!(length, items.len());
}
