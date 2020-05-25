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
| rustbus             | 31.850 us    | 74.924 us       | 53.024 us       | 396.06 us      |
| dbus-rs             | 268.66 us    | 217.19 us       | 59.626 us       | 768.94 us      |
| dbus-native         | 11.122 us    | 89.026 us       | 33.219 us       | 302.59 us      |
| dbus-bytestream     | 32.046 us    | 243.34 us       | 39.105 us       | 357.32 us      |
| dbus-message-parser | 91.640 us    | 1.2145 ms       | 230.10 us       | NaN            |
| dbus-pure           | 44.125 us    | 261.30 us       | 20.611 us       | 444.55 us      |
| zvariant            | 125.36 us    | 203.73 us       | 617.72 us       | NaN            |
