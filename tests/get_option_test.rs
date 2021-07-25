use std::path::Path;

use rust_info::Options;

#[test]
fn get() {
    let opts = Options::new().path(Some(Path::new("./").into()));
    let rust_info = rust_info::get_with_options(&opts);

    assert!(rust_info.version.is_some());
    assert!(rust_info.channel.is_some());
    assert!(rust_info.target_arch.is_some());
    assert!(rust_info.target_env.is_some());
    assert!(rust_info.target_os.is_some());
    assert!(rust_info.target_pointer_width.is_some());
    assert!(rust_info.target_triple.is_some());
}
