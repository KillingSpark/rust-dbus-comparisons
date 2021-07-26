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

I used rust 1.53.0 to run these benchmarks.

To replicate these results just run: `cargo bench`. That will run all benchmarks.

| Library             | MarshalMixed | MarshalStrArray | MarshalBigArray | Marshal + Send |
|---------------------|--------------|-----------------|-----------------|----------------|
| rustbus             | 4.0474 us    | 123.87 us       | 2.4285 us       | 91.959 us      |
| dbus-rs             | 175.22 us    | 1428.2 us       | 359.81 us       | 285.19 us      |
| dbus-native         | 5.2856 us    | 514.51 us       | 129.75 us       | 63.926 us      |
| dbus-bytestream     | 14.823 us    | 1215.0 us       | 166.74 us       | 74.898 us      |
| dbus-message-parser | 39.376 us    | 4388.5 us       | 783.96 us       | NaN            |
| dbus-pure           | 16.677 us    | 609.22 us       | 66.111 us       | 58.089 us      |
| zvariant            | 41.351 us    | 1515.1 us       | 570.14 us       | 131.65 us      |
| zvariant-derive     | 41.746 us    | 1496.4 us       | 571.17 us       | 140.48 us      |
| rustbus-async       | 4.1519 us    | 126.47 us       | 2.4595 us       | 100.62 us      |
