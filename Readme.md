# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs/ (bindings to C library)
1. https://github.com/diwic/dbus-rs/tree/master/dbus-native
1. https://gitlab.freedesktop.org/zeenix/zbus/
1. https://github.com/Arnavion/dbus-pure
1. https://github.com/srwalter/dbus-bytestream
1. https://github.com/LinkTed/dbus-message-parser

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

I used rust 1.53.0 to run these benchmarks.

To replicate these results just run: `cargo bench`. That will run all benchmarks.

| Library             | MarshalMixed | MarshalStrArray | MarshalBigArray | Marshal + Send |
|---------------------|--------------|-----------------|-----------------|----------------|
| rustbus             | 7.074 us     | 125.65 us       | 2.8302 us       | 76.774 us      |
| dbus-rs             | 177.08 us    | 1475.6 us       | 367.50 us       | 268.00 us      |
| dbus-native         | 5.2846 us    | 335.77 us       | 135.75 us       | 47.112 us      |
| dbus-bytestream     | 14.387 us    | 1213.3 us       | 166.77 us       | 62.467 us      |
| dbus-message-parser | 39.111 us    | 4491.1 us       | 784.56 us       | NaN            |
| dbus-pure           | 16.411 us    | 294.34 us       | 66.035 us       | 58.328 us      |
| zvariant            | 41.591 us    | 1493.6 us       | 567.61 us       | 117.78 us      |
| zvariant-derive     | 41.029 us    | 1504.5 us       | 567.79 us       | 119.50 us      |
