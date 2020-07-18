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
| rustbus             | 11.405 us    | 179.49 us       | 5.2076 us       | 185.19 us      |
| dbus-rs             | 165.86 us    | 1382.0 us       | 265.19 us       | 462.27 us      |
| dbus-native         | 6.3081 us    | 398.83 us       | 222.01 us       | 149.27 us      |
| dbus-bytestream     | 17.413 us    | 1356.0 us       | 153.00 us       | 182.65 us      |
| dbus-message-parser | 49.605 us    | 7845.0 us       | 1213.3 us       | NaN            |
| dbus-pure           | 18.164 us    | 362.68 us       | 52.927 us       | 225.27 us      |
| zvariant            | 29.552 us    | 1285.3 us       | 308.78 us       | NaN            |
| zvariant-derive     | 30.668 us    | 1295.5 us       | 316.89 us       | NaN            |
