//! # triple
//!
//! Manages the rust triple values and logic.
//!

#[cfg(test)]
#[path = "./triple_test.rs"]
mod triple_test;

use crate::types::RustInfo;

pub(crate) fn load(rust_info: &mut RustInfo) {
    if let (Some(arch), Some(os)) = (&rust_info.target_arch, &rust_info.target_os) {
        let mut triple = String::new();
        let mapped_arch = map_arch(arch);
        triple.push_str(&mapped_arch);

        if let Some(vendor) = &rust_info.target_vendor {
            let mapped_vendor = map_vendor(vendor, os);

            triple.push_str("-");
            triple.push_str(&mapped_vendor);
        }

        let mapped_os = map_os(os, arch);
        triple.push_str("-");
        triple.push_str(&mapped_os);

        if let Some(env) = &rust_info.target_env {
            if !env.is_empty() {
                let mapped_env = map_env(env, arch);
                triple.push_str("-");
                triple.push_str(&mapped_env);
            }
        }

        rust_info.target_triple = Some(triple);
    }
}

fn map_arch(arch: &str) -> String {
    match arch {
        "x86" => "i686".to_string(),
        _ => arch.to_string(),
    }
}

fn map_vendor(vendor: &str, os: &str) -> String {
    match os {
        "android" => "linux".to_string(),
        _ => vendor.to_string(),
    }
}

fn map_os(os: &str, arch: &str) -> String {
    match os {
        "macos" => "darwin".to_string(),
        "android" => match arch {
            "arm" => "androideabi".to_string(),
            _ => os.to_string(),
        },
        _ => os.to_string(),
    }
}

fn map_env(env: &str, arch: &str) -> String {
    match env {
        "gnu" => match arch {
            "arm" => "gnueabihf".to_string(),
            _ => env.to_string(),
        },
        _ => env.to_string(),
    }
}
