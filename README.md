# Malk

> Now with vitamin R!

I bought a [2% Milk][milk] macro board as a way to play with embedded rust.

[milk]: https://github.com/Spaceman/Spaceboards/tree/master/Keyboards/2%25%20Milk

To be clear, I have no idea what I'm doing. If this bricks anything, it's your fault for trusting randos on github.

## What it does

Right now, it blinks RX LED. This was just to make sure I could correctly
interpret the manual and read or write to the correct registers.

## How to set it up

If you use [nix][] you can use the included `shell.nix` with `nix-shell` and hack away. Otherwise you'll need an AVR GCC (for linking), a libc, and a flasher like avr-dude.

[nix]: https://nixos.org/

You can build the binary with the following `cargo` incantation. This assumes you're on nightly. You may need to use `cargo +nighty build` if you have stable installed too.

```sh
cargo build -Z build-std=core --target avr-atmega32u4.json --release
```

If it works, you can find the binary at `./target/avr-atmega32u4/release/malk.elf`

You can then flash it to your Pro Micro. I have a regular knock off, so the `avrdude-flash.sh` script helps automate finding the device.

In my experience, flashing failed more than it succeeded at first. Here are some things to try if you're having issues:

1. Some issues may be made worse by USB 3 ports according to folk wisdom.
2. Make sure your user is in the `dialout` group.
3. Reset the Pro Micro (by bridging reset to ground) twice in quick succession
   to force it into the bootloader for 8 seconds.

## Resources

Huge thanks to Philipp Opperman and his [blog_os][] post on stand-alone binaries.

[blog_os]: https://github.com/phil-opp/blog_os

The [avr-rust/blink][blink] repo has a lot of good info on getting things set up, especially the linked book, and was largely where the target `.json` file comes from. It's a little dated, in that AVR support has landed in nightly.

[blink]: https://github.com/avr-rust/blink

Also reading QMK's `shell.nix` helped me find some of the nixpkgs names of packages.

There's a great diagram of the board's pinout (crucially with what port the pins belong two) available on the [QQTrading product page][pinout].

[pinout]: http://qqtrading.com.my/arduino-pro-micro-atmega32u4-development-board

And of course reading the [manual].

[manual]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7766-8-bit-AVR-ATmega16U4-32U4_Datasheet.pdf

## License

The specific `avrdude-flash.sh` script is from [Nooges' gist][gist] and I have no idea what license it would fall under.

Other than than, this project is under the [MIT][] license.

[MIT]: https://choosealicense.com/licenses/mit
