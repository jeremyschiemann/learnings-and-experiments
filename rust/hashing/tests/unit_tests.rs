use hashing::U64Hasher;
use std::hash::{Hash, Hasher};

#[test]
fn u8_ok() {
    let mut h = U64Hasher::<u8>::default();
    let num: u8 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn u16_ok() {
    let mut h = U64Hasher::<u16>::default();
    let num: u16 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn u32_ok() {
    let mut h = U64Hasher::<u32>::default();
    let num: u32 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn u64_ok() {
    let mut h = U64Hasher::<u64>::default();
    let num: u64 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn usize_ok() {
    let mut h = U64Hasher::<usize>::default();
    let num: usize = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn i8_ok_pos() {
    let mut h = U64Hasher::<i8>::default();
    let num: i8 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn i16_ok_pos() {
    let mut h = U64Hasher::<i16>::default();
    let num: i16 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn i32_ok_pos() {
    let mut h = U64Hasher::<i32>::default();
    let num: i32 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn i64_ok_pos() {
    let mut h = U64Hasher::<i64>::default();
    let num: i64 = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn isize_ok_pos() {
    let mut h = U64Hasher::<isize>::default();
    let num: isize = 10;
    num.hash(&mut h);
    assert_eq!(10, h.finish());
}

#[test]
fn i8_ok_neg() {
    let mut h = U64Hasher::<i8>::default();
    let num: i8 = -10;
    num.hash(&mut h);
    assert_eq!(u64::MAX - 10 + 1, h.finish());
}

#[test]
fn i16_ok_neg() {
    let mut h = U64Hasher::<i16>::default();
    let num: i16 = -10;
    num.hash(&mut h);
    assert_eq!(u64::MAX - 10 + 1, h.finish());
}

#[test]
fn i32_ok_neg() {
    let mut h = U64Hasher::<i32>::default();
    let num: i32 = -10;
    num.hash(&mut h);
    assert_eq!(u64::MAX - 10 + 1, h.finish());
}

#[test]
fn i64_ok_neg() {
    let mut h = U64Hasher::<i64>::default();
    let num: i64 = -10;
    num.hash(&mut h);
    assert_eq!(u64::MAX - 10 + 1, h.finish());
}

#[test]
fn isize_ok_neg() {
    let mut h = U64Hasher::<isize>::default();
    let num: isize = -10;
    num.hash(&mut h);
    assert_eq!(u64::MAX - 10 + 1, h.finish());
}
