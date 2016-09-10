#![feature(test)]
extern crate test;
extern crate recycle;

use test::Bencher;
use recycle::*;
use std::default::Default;
use std::io::Write;

fn ret_vec<T : Default>() -> Vec<T> {
    let mut x = Vec::new();
    x.push(Default::default());
    x
}

fn ret_recycle_vec<T : Default+'static>() -> Recycle<Vec<T>>
where Vec<T> : Recyclable {
    let mut x : Recycle<Vec<T>> = Recycle::new();
    x.push(Default::default());
    x
}

#[bench]
fn alloc_vec_u8(b: &mut Bencher) {

    b.iter(|| {
        ret_vec::<u8>()
    });
}


#[bench]
fn recycle_vec_u8(b: &mut Bencher) {

    b.iter(|| {
        ret_recycle_vec::<u8>()
    });
}

#[bench]
fn alloc_vec_u64(b: &mut Bencher) {

    b.iter(|| {
        ret_vec::<u64>()
    });
}


#[bench]
fn recycle_vec_u64(b: &mut Bencher) {

    b.iter(|| {
        ret_recycle_vec::<u64>()
    });
}

const TEST_STRING : &'static str = "01234567890123456789012345678901234567890123456789";

#[bench]
fn alloc_write_vec(b: &mut Bencher) {

    b.iter(|| {
        let mut v = ret_vec::<u8>();
        let _ = write!(&mut v, "{}", TEST_STRING);
    });
}


#[bench]
fn alloc_write_recycle_vec(b: &mut Bencher) {

    b.iter(|| {
        let mut v = ret_recycle_vec::<u8>();
        let _ = write!(&mut v, "{}", TEST_STRING);
    });
}

