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
    assert!(rust_info.target_triple.is_some());
}

#[test]
fn load_triple_with_arch_first() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("unknown"));
    load_triple(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_vendor = Some(String::from("unknown"));
    load_triple(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_env = Some(String::from("unknown"));
    load_triple(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_os = Some(String::from("unknown"));
    load_triple(&mut rust_info);
    assert_eq!(
        Some(String::from("unknown-unknown-unknown-unknown")),
        rust_info.target_triple
    );
}

#[test]
fn load_triple_with_os_first() {
    let mut rust_info = RustInfo::new();

    rust_info.target_os = Some(String::from("os"));
    rust_info.target_vendor = Some(String::from("vendor"));
    rust_info.target_env = Some(String::from("env"));
    load_triple(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_arch = Some(String::from("arch"));
    load_triple(&mut rust_info);
    assert_eq!(
        Some(String::from("arch-vendor-os-env")),
        rust_info.target_triple
    );
}

#[test]
fn load_triple_with_minimum() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("arch"));
    rust_info.target_os = Some(String::from("os"));
    load_triple(&mut rust_info);
    assert_eq!(Some(String::from("arch-os")), rust_info.target_triple);
}
