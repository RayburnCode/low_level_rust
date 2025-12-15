<!-- @format -->

# Notes

[MicroBit](https://tech.microbit.org)
[Probe-rs](https://probe.rs)
Learn: Rust genetic, trait and closures

## MCU

- Nordic nRF52833
- RF is short for radio frequency
- 512 kilobyte flash and 128 kilobyte RAM
- Arm® Cortex™-M4 32-bit processor

### Terminology

- Peripheral Access Crate (PAC)

```rust
cargo add nrf52833-pac
```

- Hardware Abstraction Layer (HAL): abstracts whole peripherals away into single structs that can be used to send data around via the peripheral.

```rust
cargo add nrf52833-hal
```

- Board Support Crate (BSP)

```rust
cargo add microbit-v2
```

## Rust Tools

- gdb-multiarch
- cargo-binutils
- probe-rs-tools
- minicom

```rust
rustup component add llvm-tools
cargo install cargo-binutils
cargo binstall probe-rs-tools
```

Mac:

```sh
$ # GDB debugger - The version in brew is built for all architectures including all of the ARM embedded cores
$ brew install gdb

$ # Minicom
$ brew install minicom

$ # lsusb lists USB ports
$ brew install lsusb
```

### Generics

### Trait

A trait in Rust defines functionality that a particular type has and can share with others

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Closures

- anonymous functions that can capture variables from their surrounding scope

## Build it

```bash
cargo build
greadelf -- -h target/thumbv7em-none-eabihf/debug/micro_bit_v2
```
