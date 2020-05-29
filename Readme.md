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
| rustbus             | 24.248 us    | 423.70 us       | 289.30 us       | 386.30 us      |
| dbus-rs             | 264.98 us    | 1.9425 ms       | 401.93 us       | 805.32 us      |
| dbus-native         | 12.056 us    | 1.6035 ms       | 269.50 us       | 329.44 us      |
| dbus-bytestream     | 33.572 us    | 2.8231 ms       | 334.11 us       | 398.90 us      |
| dbus-message-parser | 92.560 us    | 15.115 ms       | 2.1936 ms       | NaN            |
| dbus-pure           | 39.192 us    | 820.21 us       | 92.196 us       | 460.46 us      |
| zvariant            | 86.469 us    | 4.3949 ms       | 1.0527 ms       | NaN            |
| zvariant-derive     | 92.644 us    | 4.4312 ms       | 1.0545 ms       | NaN            |
