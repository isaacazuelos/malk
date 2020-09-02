# Malk

![Now with vitamin R](http://static1.wikia.nocookie.net/__cb20130325185942/simpsons/images/3/3c/Malk.jpg)

I bought a [2% Milk][milk] macro board as a way to play with embedded rust.

[milk]: https://github.com/Spaceman/Spaceboards/tree/master/Keyboards/2%25%20Milk

To be clear, I have no idea what I'm doing. If this bricks anything, it's your fault for trusting randos on github.

## What it does

Right now, it does nothing. This is a placeholder for the first real code commit.

This version does "work" in that you can flash a program that just loops and does nothing. Or at least it looks like it's doing nothing.

## How to set it up

If you use [nix][] you can use the included `shell.nix` with `nix-shell` and hack away. Otherwise you'll need an AVR GCC (for linking), a libc, and a flasher like avr-dude.

[nix]: https://nixos.org/

You can build the binary with the following `cargo` incantation. This assumes you're on nightly. You may need to use `cargo +nighty build` if you have stable installed too.

```sh
cargo build -Z build-std=core --target avr-atmega32u4.json --release
```

If it works, you can find the binary at `./target/avr-atmega32u4/release/malk.elf`

You can then flash it to your Pro Micro. I have a regular knock off, so the `avrdude-flash.sh` script helps automate finding the device.

In my experience, flashing fails often. Don't forget to add yoruself to the `dialout` group on Linux. Some of the flakiness may be made worse by USB 3 ports according to folk wisdom. Just reset the Pro Micro (by bridging reset to ground) twice in quick succession to force it into the bootloader for 8 seconds. This procedure is necessary and surprisingly hard to find.

## Resources

Huge thanks to Philipp Opperman and his [blog_os][] post on stand-alone binaries.

[blog_os]: https://github.com/phil-opp/blog_os

The [avr-rust/blink][blink] repo has a lot of good info on getting things set up, especially the linked book, and was largely where the target `.json` file comes from. It's a little dated, in that AVR support has landed in nightly.

[blink]: https://github.com/avr-rust/blink

Also reading QMK's `shell.nix` helped me find some of the nixpkgs names of packages.

And of course reading the [manual].

[manual]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7766-8-bit-AVR-ATmega16U4-32U4_Datasheet.pdf

## License

The specific `avrdude-flash.sh` script is from [Nooges' gist][gist] and I have no idea what license it would fall under.

[gist]: https://gist.github.com/nooges/93560cb0c456ade5b530e95892b5e25b

Other than than, this project is under the [MIT][] license.

[MIT]: https://choosealicense.com/licenses/mit
