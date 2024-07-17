use std::hash::Hasher;
use std::marker::PhantomData;

#[derive(Default)]
pub struct U64Hasher<T>(u64, PhantomData<T>);

pub trait U64Hashable {}

macro_rules! impl_u64_hashable {
    ($($t:ty),*) => {
        $(
            impl U64Hashable for $t {}
        )*
    };
}

#[rustfmt::skip]
impl_u64_hashable!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize
);

impl<T: U64Hashable> Hasher for U64Hasher<T> {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("Types should use any of the write_{{u8, u16, ...}} functions for hashing.")
    }

    fn write_u8(&mut self, i: u8) {
        self.0 = i.into()
    }

    fn write_u16(&mut self, i: u16) {
        self.0 = i.into()
    }

    fn write_u32(&mut self, i: u32) {
        self.0 = i.into()
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i
    }

    fn write_usize(&mut self, i: usize) {
        self.0 = i as u64
    }

    fn write_i8(&mut self, i: i8) {
        self.0 = i as u64
    }

    fn write_i16(&mut self, i: i16) {
        self.0 = i as u64
    }

    fn write_i32(&mut self, i: i32) {
        self.0 = i as u64
    }

    fn write_i64(&mut self, i: i64) {
        self.0 = i as u64
    }

    fn write_isize(&mut self, i: isize) {
        self.0 = i as u64
    }
}
