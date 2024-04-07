use inplace_sorting;

fn main() {
    use std::time::Instant;

    let mut big_vec: Vec<i32> = Vec::with_capacity(100000);

    for _ in 0..big_vec.capacity() {
        big_vec.push(rand::random());
    }

    let mut copied = big_vec.clone();

    let now = Instant::now();
    {
        copied.sort();
    }
    let elapsed = now.elapsed();
    println!("std::sorting took: {elapsed:?}");

    let mut copied = big_vec.clone();

    let now = Instant::now();
    {
        copied.sort_unstable();
    }
    let elapsed = now.elapsed();
    println!("std::sorting unstable took: {elapsed:?}");

    let mut copied = big_vec.clone();

    let now = Instant::now();
    {
        inplace_sorting::bubble_sort(copied.as_mut_slice());
    }
    let elapsed = now.elapsed();
    println!("bubblesort took: {elapsed:?}");
}
