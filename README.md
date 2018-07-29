# betafpv-f3

> Board Support Crate for the BetaFPV F3 Drone Flight Controller

## What works

- LED control
- Motor control
- Serial communication
- Basic gyro/accelerometer functionality
- A script to build each example and flash it onto the controller 

## TODO

- [ ] Sensor fusion
- [ ] Radio receiver
- [ ] USB port support

## Use

To build the led-control example and flash it onto a BetaFPV F3 board, first plug the board into your computer while holding down the `BOOT` button to enable bootloader mode, then run the following command:

```bash
    ./flash-example led-control
```
    
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
