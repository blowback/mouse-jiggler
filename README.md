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


