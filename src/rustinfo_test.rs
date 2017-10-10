use super::*;
use std::io::ErrorKind;

#[test]
fn get_exit_code_error() {
    let code = get_exit_code(Err(Error::new(ErrorKind::Other, "test")));
    assert_eq!(code, -1);
}

#[test]
fn load_with_values() {
    let rust_info = get();

    assert!(rust_info.version.is_some());
    assert!(rust_info.channel.is_some());
    assert!(rust_info.target_arch.is_some());
    assert!(rust_info.target_env.is_some());
    assert!(rust_info.target_os.is_some());
    assert!(rust_info.target_pointer_width.is_some());
    assert!(rust_info.target_vendor.is_some());
}
