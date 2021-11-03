Rust Examples for the DERULER project 


## Quickstart
Install Rust:
https://www.rust-lang.org/tools/install

install GNU Arm Toolchain:
https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

Install Target for Cortex-M0+
```shell
rustup target add thumbv6m-none-eabi
```
Install probe-run
```shell
cargo install probe-run
```

To build, flash and run an example
```shell
cargo r -p <name_of_example>
```
