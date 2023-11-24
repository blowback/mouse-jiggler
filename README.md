# ESP32 BLE Mouse Jiggler

A device which periodically randomly perturbs the mouse, for keeping you active
on chat media, stopping screen savers kick in etc etc

# setup

```
. ~/export-esp.sh
cargo generate esp-rs/esp-idf-template cargo     # simple templates, target esp32
cargo add esp-idf-hal esp-idf-sys anyhow esp32-nimble
```

# build

```
cargo run
```

# use

Flash an ESP32 board (I used an AdaFruit Huzzah ESP32, but any variant should work) with the code:

```
cargo run
```

I built mine into an old film canister and powered it from the monitor's USB port:
![2023-11-24 10 53 58](https://github.com/blowback/mouse-jiggler/assets/320453/5bf399b1-1487-4202-a08a-d76b9eaaf630)
![2023-11-24 10 55 20](https://github.com/blowback/mouse-jiggler/assets/320453/51e1b4c2-0599-4ef3-989e-06fbc9ce7fad)



Then on your target computer, enable Bluetooth and pair with "BT Mouse". For some reason it later
changes its name to "nimble" on a mac, not sure what that is all about.

Every 60 seconds, it will give your mouse a little jiggle!


https://github.com/blowback/mouse-jiggler/assets/320453/972017fd-2e62-495c-a250-6ded575088e6


If you monitor the debug console you'll see this when it jiggles:

![image](https://github.com/blowback/mouse-jiggler/assets/320453/64db2bbb-dd6a-4522-99b1-7a05a393c95b)

To change the interval, edit the timeout in the last line of `src/main.rs`.

