<!-- @format -->

# Blog post 4: Testing

- Rust has a built-in test framework that can run unit tests without the need to set anything up
- Rust’s test framework uses the built-in test library, which depends on the standard library. Which isnt' possible with using #[no_std]

## Notes

- Memory Mapped I/O vs Port Mapped I/O
- port-mapped I/O uses a separate I/O bus for communication.
- CPU instructions called in and out, which take a port number and a data byte (there are also variations of these commands that allow sending a u16 or u32)

### UART

- Universal asynchronous receiver-transmitter
- Data dent in one by one bits, from the least to most significant
- Common signal levels are RS-232, RS-485, and raw TTL(Transistor-Transistor Logic)
