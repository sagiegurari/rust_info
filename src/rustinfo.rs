//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use std::collections::HashMap;
use std::io::Error;
use std::process::{Command, ExitStatus};
use types::{RustChannel, RustInfo};

/// Returns the exit code (-1 if no exit code found)
fn get_exit_code(exit_status: Result<ExitStatus, Error>) -> i32 {
    match exit_status {
        Ok(code) => {
            if !code.success() {
                match code.code() {
                    Some(value) => value,
                    None => -1,
                }
            } else {
                0
            }
        }
        _ => -1,
    }
}

fn load_version(rust_info: &mut RustInfo) {
    let result = Command::new("rustc").arg("--version").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = stdout.split(' ').collect();

                if (parts.len() >= 3) && (parts[0] == "rustc") {
                    let version_part = parts[1];

                    let version_parts: Vec<&str> = version_part.split('-').collect();

                    if version_parts.len() > 0 {
                        rust_info.version = Some(version_parts[0].to_string());

                        if version_parts.len() == 1 {
                            rust_info.channel = Some(RustChannel::Stable);
                        } else if version_parts[1].contains("beta") {
                            rust_info.channel = Some(RustChannel::Beta);
                        } else if version_parts[1].contains("nightly") {
                            rust_info.channel = Some(RustChannel::Nightly);
                        }
                    }
                }
            }
        }
        _ => (),
    };
}

fn load_setup(rust_info: &mut RustInfo) {
    let result = Command::new("rustc").arg("--print").arg("cfg").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let mut values = HashMap::<String, String>::new();

                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    if line.contains("=") {
                        let parts: Vec<&str> = line.split('=').collect();
                        let value = str::replace(parts[1], "\"", "");
                        values.insert(parts[0].to_string(), value.to_string());
                    }
                }

                rust_info.target_arch = Some(values.remove("target_arch").unwrap_or("unknown".to_string()));
                rust_info.target_env = Some(values.remove("target_env").unwrap_or("unknown".to_string()));
                rust_info.target_os = Some(values.remove("target_os").unwrap_or("unknown".to_string()));
                rust_info.target_pointer_width = Some(values.remove("target_pointer_width").unwrap_or("unknown".to_string()));
                rust_info.target_vendor = Some(values.remove("target_vendor").unwrap_or("unknown".to_string()));
            }
        }
        _ => (),
    };
}

/// Loads and returns the current rust compiler version and setup.<br>
/// In case partial data is not available, those values will be set to Option::None.
///
/// # Example
///
/// ```extern crate rust_info;
///
/// fn main() {
///     let rust_info = rust_info::get();
///
///     println!("Version: {}", rust_info.version.unwrap());
///     println!("Channel: {:#?}", rust_info.channel.unwrap());
///     println!("Target Arch: {}", rust_info.target_arch.unwrap());
///     println!("Target Env: {}", rust_info.target_env.unwrap());
///     println!("Target OS: {}", rust_info.target_os.unwrap());
///     println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap());
///     println!("Target Vendor: {}", rust_info.target_vendor.unwrap());
/// }
/// ```
pub fn get() -> RustInfo {
    let mut rust_info = RustInfo::new();

    load_version(&mut rust_info);

    load_setup(&mut rust_info);

    rust_info
}
