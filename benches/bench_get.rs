#![feature(test)]
extern crate test;

use rust_info;
use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let rust_info = rust_info::get();

        assert!(rust_info.version.is_some());
        assert!(rust_info.channel.is_some());
    });
}
