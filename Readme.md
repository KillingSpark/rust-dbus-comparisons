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

The Marshal + Send benchmark is not performed for zvariant because the zbus library currently uses a lot of println!()
which hamper performance unfairly. The dbus-message-parser does not provide any means of sending messages.

## Current results
I am running this on an older AMD Cpu (/proc/cpuinfo says: AMD FX(tm)-6300 Six-Core Processor). Your values might vary a bit.

To replicate these results just run: `cargo bench`. That will run all benchmarks. Alternatively you can rerun the benchmarks with more samples to get
more reliable results. I used these parameters on the AMD CPU: `target/release/deps/marshal_bench-221b1ccb00ad3f0a --nresamples 1000 --sample-size 1000 --bench`.

| Library             | MarshalMixed | MarshalStrArray | MarshalBigArray | Marshal + Send |
|---------------------|--------------|-----------------|-----------------|----------------|
| rustbus             | 21.154 us    | 261.09 us       | 231.50 us       | 396.06 us      |
| dbus-rs             | 267.46 us    | 1.9195 ms       | 400.50 us       | 768.94 us      |
| dbus-native         | 11.985 us    | 1.5734 ms       | 264.71 us       | 302.59 us      |
| dbus-bytestream     | 31.704 us    | 2.7052 ms       | 337.41 us       | 357.32 us      |
| dbus-message-parser | 90.922 us    | 15.061 ms       | 2.2038 ms       | NaN            |
| dbus-pure           | 38.783 us    | 810.59 us       | 86.795 us       | 444.55 us      |
| zvariant            | 96.278 us    | 5.1154 ms       | 1.0149 ms       | NaN            |
| zvariant-derive     | 97.254 us    | 4.9842 ms       | 1.0151 ms       | NaN            |
