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
To replicate these results just run: cargo bench. That will run all benchmarks.

| Library             | Marshal                                 | Marshal + Send                          |
|---------------------|-----------------------------------------|-----------------------------------------|
| rustbus             | time:   [88.845 us 89.417 us 90.363 us] | time:   [531.11 us 537.78 us 544.82 us] |
| zvariant            | time:   [77.969 us 78.156 us 78.354 us] |                                         |
| dbus-rs             | time:   [499.33 us 502.18 us 505.45 us] | time:   [1.2374 ms 1.2951 ms 1.3533 ms] |
| dbus-bytestream     | time:   [64.859 us 65.127 us 65.439 us] | time:   [462.18 us 469.83 us 478.73 us] |
| dbus-message-parser | time:   [171.56 us 172.28 us 173.10 us] |                                         |
| dbus-pure           | time:   [101.80 us 102.22 us 102.73 us] |                                         |
