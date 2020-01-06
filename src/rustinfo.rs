//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use crate::types::{RustChannel, RustInfo};
use std::collections::HashMap;
use std::io::Error;
use std::process::{Command, ExitStatus};

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

                let mut value = values.remove("target_arch");
                if value.is_some() {
                    rust_info.target_arch = Some(value.unwrap());
                }

                value = values.remove("target_env");
                if value.is_some() {
                    rust_info.target_env = Some(value.unwrap());
                }

                value = values.remove("target_os");
                if value.is_some() {
                    rust_info.target_os = Some(value.unwrap());
                }

                value = values.remove("target_pointer_width");
                if value.is_some() {
                    rust_info.target_pointer_width = Some(value.unwrap());
                }

                value = values.remove("target_vendor");
                if value.is_some() {
                    rust_info.target_vendor = Some(value.unwrap());
                }
            }
        }
        _ => (),
    };
}

fn load_triple(rust_info: &mut RustInfo) {
    let mut triple = String::new();

    if let Some(arch) = &rust_info.target_arch {
        triple.push_str(arch);
    } else {
        triple.push_str("unknown")
    }

    if let Some(vendor) = &rust_info.target_vendor {
        triple.push_str("-");
        triple.push_str(vendor);
    }

    if let Some(os) = &rust_info.target_os {
        triple.push_str("-");
        triple.push_str(os);
    }

    if let Some(env) = &rust_info.target_env {
        triple.push_str("-");
        triple.push_str(env);
    }

    if triple != "unknown" {
        rust_info.target_triple = Some(triple);
    }
}

/// Loads and returns the current rust compiler version and setup.<br>
/// In case partial data is not available, those values will be set to Option::None.
pub(crate) fn get() -> RustInfo {
    let mut rust_info = RustInfo::new();

    load_version(&mut rust_info);

    load_setup(&mut rust_info);

    load_triple(&mut rust_info);

    rust_info
}
