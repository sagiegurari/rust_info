# rust_info

[![crates.io](https://img.shields.io/crates/v/rust_info.svg)](https://crates.io/crates/rust_info) [![Continuous Integration](https://github.com/sagiegurari/rust_info/workflows/Continuous%20Integration/badge.svg?branch=master)](https://github.com/sagiegurari/rust_info/actions) [![codecov](https://codecov.io/gh/sagiegurari/rust_info/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/rust_info)<br>
[![license](https://img.shields.io/crates/l/rust_info.svg)](https://github.com/sagiegurari/rust_info/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/rust_info.svg)](https://libraries.io/cargo/rust_info) [![Documentation](https://docs.rs/rust_info/badge.svg)](https://docs.rs/crate/rust_info/) [![downloads](https://img.shields.io/crates/d/rust_info.svg)](https://crates.io/crates/rust_info)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Extracts the current [rust](https://www.rust-lang.org/) compiler information.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/rust_info/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current rust installation and setup.

<a name="usage"></a>
## Usage
Simply include the library and invoke the get function to pull all info as follows:

````rust
fn main() {
    let rust_info = rust_info::get();

    println!("Version: {}", rust_info.version.unwrap());
    println!("Channel: {:#?}", rust_info.channel.unwrap());
    println!("Target Arch: {}", rust_info.target_arch.unwrap_or("unknown".to_string()));
    println!("Target Env: {}", rust_info.target_env.unwrap_or("unknown".to_string()));
    println!("Target OS: {}", rust_info.target_os.unwrap_or("unknown".to_string()));
    println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap_or("unknown".to_string()));
    println!("Target Vendor: {}", rust_info.target_vendor.unwrap_or("unknown".to_string()));
    println!("Target Triple: {}", rust_info.target_triple.unwrap_or("unknown".to_string()));
}
````

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
rust_info = "*"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/rust_info/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2019-01-06  | v0.3.0  | Adding target_triple information. |
| 2017-10-10  | v0.1.1  | Update exposed types. |
| 2017-10-10  | v0.1.0  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
