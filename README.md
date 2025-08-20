# Wake On Lan with HomeKit

Connect the Raspberry Pi to the PC through a network cable, and connect the Raspberry Pi to the home Wifi, then the PC can be woken up through HomeKit.

Useful when you don't want to push the power button of your PC. For example, you use your laptop with its lid closed, and it does not connected to the router with wire, and you didn't managed to make WOWLAN work.

For simplicity, I did not expose the configurable interface, you need to modify the constants in the code and compile it yourself.

## Known Issue

The `hap-rs` has [a known issue here](https://github.com/ewilken/hap-rs/issues/90), inside the comments there is a workaround solution.

Update @ 2025:
Note that the `hap-rs` crate appears to be no longer under development. I [forked](https://github.com/ihciah/hap-rs) it and fixed some dependency and leak issues, but I don't plan to actively maintain it.

## Why not Use Cheaper Hardware

1. I have a bunch of boards like Raspberry Pi.
2. An ESP32 with ethernet is relatively expensive.
3. It's simpler to program on linux.
