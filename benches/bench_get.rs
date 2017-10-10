#![feature(test)]
extern crate rust_info;
extern crate test;

use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let rust_info = rust_info::get();

        assert!(rust_info.version.is_some());
        assert!(rust_info.channel.is_some());
        assert!(rust_info.target_arch.is_some());
        assert!(rust_info.target_env.is_some());
        assert!(rust_info.target_os.is_some());
        assert!(rust_info.target_pointer_width.is_some());
        assert!(rust_info.target_vendor.is_some());
    });
}
