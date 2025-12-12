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
