<!-- @format -->

# low_level_rust

From Philipp Oppermann's blog

## Tools

- Rust
- QEMU

create a bootable disk image:

```rust
cargo bootimage
```

```rust
cd /Users/DylanRayburn/Documents/GitHub/low_level_rust/bare_metal
qemu-system-x86_64 -drive format=raw,file=target/x86_64-bare_metal/debug/bootimage-bare_metal.bin
```

### Embedded-HAL

Some of the defined traits in embedded-hal are:

GPIO (input and output pins)
Serial communication
I2C
SPI
Timers/Countdowns
Analog Digital Conversion
