# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs/ (bindings to C library)
1. https://github.com/dbus2/zbus/
1. https://github.com/cmaves/async-rustbus
1. https://github.com/gtk-rs/gtk-rs-core

Note that I am the author of rustbus, but of course I am trying to be as objective as possible here.

## Current state
Some benchmarks exist. I plan to add equivalent ones for the missing libs, and more kinds of benchmarks.

## The benchmarks
1. MarshalMix: Build a signal message with mixed params and marshal it
1. MarshalBigArray: Build a signal message with a big u64 array and marshal it
1. MarshalStrArray: Build a signal message with a big String array and marshal it
1. Marshal + Send: Connect to the sessionbus, build a signal and send it to the bus

The dbus-message-parser does not provide any means of sending messages, so this benchmark is omitted.

## Current results
I am running this on a Ryzen 3800X (/proc/cpuinfo says: AMD Ryzen 7 3800X). Your values might vary a bit.

I used `rustc 1.95.0 (59807616e 2026-04-14)` to run these benchmarks.

To replicate these results just run: `cargo bench`. That will run all benchmarks.

| Library             | MarshalMixed | MarshalBigStrArray | MarshalBigArray | Marshal + Send |
|---------------------|--------------|--------------------|-----------------|----------------|
| rustbus             | 3.4442 µs    | 98.406 µs          | 2.2243 µs       | 127.45 µs      |
| dbus-rs             | 169.14 µs    | 1.9275 ms          | 393.13 µs       | 285.30 µs      |
| zvariant            | 9.5993 µs    | 229.79 µs          | 75.775 µs       | 145.99 µs      |
| zvariant-derive     | 9.6960 µs    | 231.98 µs          | 79.295 µs       | 147.36 µs      |
| rustbus-async       | 4.1619 µs    | 104.90 µs          | 2.2733 µs       | 149.61 µs      |
| glib                | 35.476 µs    | 2.8925 ms          | 1.1315 ms       | 215.65 µs      |
