use super::*;

#[test]
fn load_with_arch_first() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("unknown"));
    load(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_vendor = Some(String::from("unknown"));
    load(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_env = Some(String::from("unknown"));
    load(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_os = Some(String::from("unknown"));
    load(&mut rust_info);
    assert_eq!(
        Some(String::from("unknown-unknown-unknown-unknown")),
        rust_info.target_triple
    );
}

#[test]
fn load_with_os_first() {
    let mut rust_info = RustInfo::new();

    rust_info.target_os = Some(String::from("os"));
    rust_info.target_vendor = Some(String::from("vendor"));
    rust_info.target_env = Some(String::from("env"));
    load(&mut rust_info);
    assert!(rust_info.target_triple.is_none());

    rust_info.target_arch = Some(String::from("arch"));
    load(&mut rust_info);
    assert_eq!(
        Some(String::from("arch-vendor-os-env")),
        rust_info.target_triple
    );
}

#[test]
fn load_with_minimum() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("arch"));
    rust_info.target_os = Some(String::from("os"));
    load(&mut rust_info);
    assert_eq!(Some(String::from("arch-os")), rust_info.target_triple);
}

#[test]
fn load_with_empty_env() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("arch"));
    rust_info.target_os = Some(String::from("os"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(Some(String::from("arch-os")), rust_info.target_triple);
}

#[test]
fn load_mac_32bit() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86"));
    rust_info.target_os = Some(String::from("macos"));
    rust_info.target_vendor = Some(String::from("apple"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "i686-apple-darwin");
}

#[test]
fn load_mac_64bit() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("macos"));
    rust_info.target_vendor = Some(String::from("apple"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-apple-darwin");
}

#[test]
fn load_windows_32bit_gnu() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86"));
    rust_info.target_os = Some(String::from("windows"));
    rust_info.target_vendor = Some(String::from("pc"));
    rust_info.target_env = Some(String::from("gnu"));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "i686-pc-windows-gnu");
}

#[test]
fn load_window_64bit_gnu() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("windows"));
    rust_info.target_vendor = Some(String::from("pc"));
    rust_info.target_env = Some(String::from("gnu"));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-pc-windows-gnu");
}

#[test]
fn load_windows_32bit_msvc() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86"));
    rust_info.target_os = Some(String::from("windows"));
    rust_info.target_vendor = Some(String::from("pc"));
    rust_info.target_env = Some(String::from("msvc"));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "i686-pc-windows-msvc");
}

#[test]
fn load_window_64bit_msvc() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("windows"));
    rust_info.target_vendor = Some(String::from("pc"));
    rust_info.target_env = Some(String::from("msvc"));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-pc-windows-msvc");
}

#[test]
fn load_linux_32bit_gnu() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86"));
    rust_info.target_os = Some(String::from("linux"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from("gnu"));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "i686-unknown-linux-gnu");
}

#[test]
fn load_linux_64bit_gnu() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("linux"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from("gnu"));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-unknown-linux-gnu");
}

#[test]
fn load_linux_64bit_musl() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("linux"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from("musl"));
    load(&mut rust_info);
    assert_eq!(
        rust_info.target_triple.unwrap(),
        "x86_64-unknown-linux-musl"
    );
}

#[test]
fn load_linux_arm_gnu() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("arm"));
    rust_info.target_os = Some(String::from("linux"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from("gnu"));
    load(&mut rust_info);
    assert_eq!(
        rust_info.target_triple.unwrap(),
        "arm-unknown-linux-gnueabihf"
    );
}

#[test]
fn load_android_32bit() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86"));
    rust_info.target_os = Some(String::from("android"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "i686-linux-android");
}

#[test]
fn load_android_64bit() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("android"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-linux-android");
}

#[test]
fn load_android_arm() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("arm"));
    rust_info.target_os = Some(String::from("android"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "arm-linux-androideabi");
}

#[test]
fn load_freebsd_64bit() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("freebsd"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-unknown-freebsd");
}

#[test]
fn load_netbsd_64bit() {
    let mut rust_info = RustInfo::new();

    rust_info.target_arch = Some(String::from("x86_64"));
    rust_info.target_os = Some(String::from("netbsd"));
    rust_info.target_vendor = Some(String::from("unknown"));
    rust_info.target_env = Some(String::from(""));
    load(&mut rust_info);
    assert_eq!(rust_info.target_triple.unwrap(), "x86_64-unknown-netbsd");
}
