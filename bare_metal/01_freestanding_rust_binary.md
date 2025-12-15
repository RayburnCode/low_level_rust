<!-- @format -->

# Blog one notes

no threads, files, heap memory, the network, random numbers, standard output,

1. Target Tripple of the host operating system
   ex: **host: x86_64-unknown-linux-gnu**
   CPU architecture (x86_64), the vendor (unknown), the operating system (linux), and the ABI (gnu)
   ABI = Application Binary Interface

```rust
rustc --version --verbose
```

Must compile for a different environment with no underlying operating system
ex:

```rust
rustup target add thumbv7em-none-eabihf
```

build

```rust
cargo build --target thumbv7em-none-eabihf
```

## Final Code

```rust
// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}
```
