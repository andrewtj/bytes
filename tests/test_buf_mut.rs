#![deny(warnings, rust_2018_idioms)]

use bytes::{buf::IoSliceMut, BufMut, BytesMut};
use std::usize;
use std::fmt::Write;

#[test]
fn test_vec_as_mut_buf() {
    let mut buf = Vec::with_capacity(64);

    assert_eq!(buf.remaining_mut(), usize::MAX);

    assert!(buf.bytes_mut().len() >= 64);

    buf.put(&b"zomg"[..]);

    assert_eq!(&buf, b"zomg");

    assert_eq!(buf.remaining_mut(), usize::MAX - 4);
    assert_eq!(buf.capacity(), 64);

    for _ in 0..16 {
        buf.put(&b"zomg"[..]);
    }

    assert_eq!(buf.len(), 68);
}

#[test]
fn test_put_u8() {
    let mut buf = Vec::with_capacity(8);
    buf.put_u8(33);
    assert_eq!(b"\x21", &buf[..]);
}

#[test]
fn test_put_u16() {
    let mut buf = Vec::with_capacity(8);
    buf.put_u16(8532);
    assert_eq!(b"\x21\x54", &buf[..]);

    buf.clear();
    buf.put_u16_le(8532);
    assert_eq!(b"\x54\x21", &buf[..]);
}

#[test]
fn test_vec_advance_mut() {
    // Regression test for carllerche/bytes#108.
    let mut buf = Vec::with_capacity(8);
    unsafe {
        buf.advance_mut(12);
        assert_eq!(buf.len(), 12);
        assert!(buf.capacity() >= 12, "capacity: {}", buf.capacity());
    }
}

#[test]
fn test_clone() {
    let mut buf = BytesMut::with_capacity(100);
    buf.write_str("this is a test").unwrap();
    let buf2 = buf.clone();

    buf.write_str(" of our emergency broadcast system").unwrap();
    assert!(buf != buf2);
}

#[test]
fn test_bufs_vec_mut() {
    let b1: &mut [u8] = &mut [];
    let b2: &mut [u8] = &mut [];
    let mut dst = [IoSliceMut::from(b1), IoSliceMut::from(b2)];

    // with no capacity
    let mut buf = BytesMut::new();
    assert_eq!(buf.capacity(), 0);
    assert_eq!(0, buf.bytes_vectored_mut(&mut dst[..]));

    // with capacity
    let mut buf = BytesMut::with_capacity(64);
    assert_eq!(1, buf.bytes_vectored_mut(&mut dst[..]));
}

#[test]
fn test_mut_slice() {
    let mut v = vec![0, 0, 0, 0];
    let mut s = &mut v[..];
    s.put_u32(42);
}
