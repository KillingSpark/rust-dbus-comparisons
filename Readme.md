# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs/ (bindings to C library)
1. https://github.com/diwic/dbus-rs/tree/master/dbus-native
1. https://gitlab.freedesktop.org/zeenix/zbus/
1. https://github.com/Arnavion/dbus-pure
1. https://github.com/srwalter/dbus-bytestream
1. https://github.com/LinkTed/dbus-message-parser
1. https://github.com/cmaves/async-rustbus

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

I used `rustc 1.65.0 (897e37553 2022-11-02)` to run these benchmarks.

To replicate these results just run: `cargo bench`. That will run all benchmarks.

| Library             | MarshalMixed | MarshalStrArray | MarshalBigArray | Marshal + Send |
|---------------------|--------------|-----------------|-----------------|----------------|
| rustbus             | 3.4257 µs    | 94.031 µs       | 2.1689 µs       | 71.926 µs      |
| dbus-rs             | 168.82 µs    | 1.3286 ms       | 377.54 µs       | 262.12 µs      |
| dbus-native         | 5.1487 µs    | 551.10 µs       | 123.87 µs       | 49.769 µs      |
| dbus-bytestream     | 14.002 µs    | 1.1528 ms       | 134.56 µs       | 64.110 µs      |
| dbus-message-parser | 35.968 µs    | 4.6027 ms       | 756.30 µs       | NaN            |
| dbus-pure           | 14.657 µs    | 263.60 µs       | 19.066 µs       | 53.806 µs      |
| zvariant            | 54.555 µs    | 1.0492 ms       | 538.07 µs       | 246.59 µs      |
| zvariant-derive     | 55.633 µs    | 1.0560 ms       | 545.26 µs       | 245.16 µs      |
| rustbus-async       | 4.5583 µs    | 145.70 µs       | 2.2329 µs       | 116.30 µs      |
