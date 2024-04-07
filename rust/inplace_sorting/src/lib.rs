pub fn bubble_sort<T>(slice: &mut [T])
where
    T: PartialOrd,
{
    for idx in 0..slice.len() - 1 {
        for idx2 in 0..slice.len() - 1 - idx {
            if slice[idx2] > slice[idx2 + 1] {
                slice.swap(idx2, idx2 + 1);
            }
        }
    }
}
