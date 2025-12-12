<!-- @format -->

# Blog Post 2

Need to set the version to Nightly:

```rust
rustup override set nightly
```

verify version:

```rust
rustc --version
```

BIOS sits on ROM

On x86, there are two firmware standards:

1. the “Basic Input/Output System“ (BIOS) and
2. the newer “Unified Extensible Firmware Interface” (UEFI)

## Bootloader

- <https://github.com/rust-osdev/bootimage>
- automatically prepends a bootloader to your kernel.
- Open Bootloader Standard - Multiboot <https://wiki.osdev.org/Multiboot>

### GNU GRUB

- Grand Unified Bootloader
- Most popular bootloader for Linux systems.

##

```rust
rustup component add rust-src
```

Blog OS Code:

```rust
cargo build --target x86_64-blog_os.json
```

ours:

```rust
cargo build --target x86_64-bare_metal.json
```

```rust
cargo build --target arm-bare_metal.json
```
