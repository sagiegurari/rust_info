extern crate rust_info;

fn main() {
    let rust_info = rust_info::get();

    println!("Version: {}", rust_info.version.unwrap());
    println!("Channel: {:#?}", rust_info.channel.unwrap());
    println!("Target Arch: {}", rust_info.target_arch.unwrap());
    println!("Target Env: {}", rust_info.target_env.unwrap());
    println!("Target OS: {}", rust_info.target_os.unwrap());
    println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap());
    println!("Target Vendor: {}", rust_info.target_vendor.unwrap());
}
