# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs
1. https://gitlab.freedesktop.org/zeenix/zbus/
1. https://github.com/Arnavion/dbus-pure
1. https://github.com/srwalter/dbus-bytestream
1. https://github.com/LinkTed/dbus-message-parser

Note that I am the author of rustbus, but of course I am trying to be as objectiv as possible here.

## Current state
Some benchmarks exist. I plan to add equivalent ones for the missing libs, and more kinds of benchmarks.

## The benchmarks
1. Marshal: Build a signal message and marshal it
1. Marshal + Send: Connect to the sessionbus, build a signal and send it to the bus

The Marshal + Send benchmark is not performed for zvariant because the zbus library currently uses a lot of println!() 
which hamper performance unfairly. The dbus-message-parser does not provide any means of sending messages.

## Current results
I am running this on an older AMD Cpu (/proc/cpuinfo says: AMD FX(tm)-6300 Six-Core Processor). Your values might vary a bit.

To replicate these results just run: `cargo bench`. That will run all benchmarks.

| Library             | Marshal                                 | Marshal + Send                          |
|---------------------|-----------------------------------------|-----------------------------------------|
| dbus-bytestream     | time:   [64.859 us 65.127 us 65.439 us] | time:   [462.18 us 469.83 us 478.73 us] |
| rustbus             | time:   [69.894 us 69.970 us 70.048 us] | time:   [548.01 us 572.15 us 598.81 us] |
| zvariant            | time:   [77.969 us 78.156 us 78.354 us] |                                         |
| dbus-pure           | time:   [101.80 us 102.22 us 102.73 us] | time:   [563.60 us 570.37 us 577.78 us] |
| dbus-message-parser | time:   [171.56 us 172.28 us 173.10 us] |                                         |
| dbus-rs             | time:   [499.33 us 502.18 us 505.45 us] | time:   [1.2374 ms 1.2951 ms 1.3533 ms] |

Results on a mobile Intel cpu (/proc/cpuinfo says: Intel(R) Pentium(R) 3556U @ 1.70GHz)
| Library             | Marshal                                 | Marshal + Send                          |
|---------------------|-----------------------------------------|-----------------------------------------|
| zvariant            | time:   [92.436 us 93.289 us 94.121 us] |                                         |
| dbus-bytestream     | time:   [93.544 us 93.586 us 93.650 us] | time:   [985.33 us 988.87 us 992.29 us] |
| rustbus             | time:   [95.913 us 96.014 us 96.145 us] | time:   [1.0043 ms 1.0136 ms 1.0212 ms] |
| dbus-pure           | time:   [113.36 us 113.75 us 114.27 us] | time:   [1.2606 ms 1.2741 ms 1.2884 ms] |
| dbus-message-parser | time:   [223.25 us 223.35 us 223.55 us] |                                         |
| dbus-rs             | time:   [686.15 us 688.88 us 692.73 us] | time:   [1.5581 ms 1.5659 ms 1.5763 ms] |
