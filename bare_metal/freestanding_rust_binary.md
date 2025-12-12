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

2. Must compile for a different environment with no underlying operating system
   ex:

```rust
rustup target add thumbv7em-none-eabihf
```

build

```rust
cargo build --target thumbv7em-none-eabihf
```
