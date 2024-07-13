use std::hint::black_box;

use bytes::{Bytes, BytesMut};

const LONG: &[u8] = b"mary had a little lamb, little lamb, little lamb";

// pub fn main() {
//     let mut b = BytesMut::from(LONG);
//     // sharedにpromote
//     let mut c = b.split().freeze();
//     let s = c.clone();
//     let a = s.clone();

//     black_box(a);
// }

// pub fn main() {
//     let mut b = BytesMut::from(LONG);
//     let c = b.split().freeze();
//     let b = c.try_into_mut();
//     // assert!(!c.is_unique());
//     // drop(b);
//     // assert!(c.is_unique());
// }

pub fn main() {
    let bytes = Bytes::from(b"hello".to_vec());
    let s = bytes.try_into_mut().unwrap();
    black_box(s);
}
