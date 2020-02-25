# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs
1. https://gitlab.freedesktop.org/zeenix/zbus/
1. https://github.com/Arnavion/dbus-pure
1. https://github.com/srwalter/dbus-bytestream

Note that I am the author of rustbus, but of course I am trying to be as objectiv as possible here.

## Current state
Benchmarks for rustbus and dbus-rs exist. I plan to add equivalent ones for the others.

## The benchmarks
1. Marshal: Build a signal message and marshal it
1. Marshal + Send: Connect to the sessionbus, build a signal and send it to the bus

## Current results
To replicate these results just run: cargo bench. That will run all benchmarks.



| Library | Marshal                                 | Marshal + Send                  |
|---------|-----------------------------------------|---------------------------------|
| rustbus | time:   [41.016 us 41.042 us 41.068 us] | [373.74 us 380.67 us 387.95 us] |
| dbus-rs | time:   [249.86 us 250.22 us 250.63 us] | [1.1603 ms 1.2399 ms 1.3128 ms] |