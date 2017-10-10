use super::*;

#[test]
fn rust_info_new() {
    let rust_info = RustInfo::new();

    assert!(rust_info.version.is_none());
    assert!(rust_info.channel.is_none());
    assert!(rust_info.target_arch.is_none());
    assert!(rust_info.target_env.is_none());
    assert!(rust_info.target_os.is_none());
    assert!(rust_info.target_pointer_width.is_none());
    assert!(rust_info.target_vendor.is_none());
}
