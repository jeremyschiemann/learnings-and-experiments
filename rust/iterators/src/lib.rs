use std::ops::Add;
pub trait Doubled: Iterator {
    fn doubled(self) -> DoubledIter<Self>
    where
        Self: Sized,
    {
        DoubledIter { iter: self }
    }
}

impl<I> Doubled for I where I: Iterator {}

pub struct DoubledIter<I> {
    iter: I,
}

impl<I> Iterator for DoubledIter<I>
where
    I: Iterator,
    I::Item: Copy + Add<Output = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4, 5]
            .into_iter()
            .doubled()
            .map(|x| x * 2)
            .doubled()
            .collect::<Vec<_>>();
        assert_eq!(v, vec![8, 16, 24, 32, 40]);
    }
}
