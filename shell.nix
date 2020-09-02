let 
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

  avrlibc = nixpkgs.pkgsCross.avr.buildPackages.libcCross;
in

nixpkgs.mkShell {
  name = "malk";

  buildInputs = with nixpkgs; [
    # As far as I can tell, this would be the _right_ way, but it
    # doesn't work. Which is why rustup is enables instead.
    #rustup # don't forget to use `cargo +nightly` if you have stable installed.
    ((rustChannelOf {
     date = "2020-08-01";
     channel = "nightly";
    }).rust.override {
     extensions = [ "rust-src" ];
     targetExtensions = [ "rust-src" ];
    })
    pkgsCross.avr.buildPackages.gcc8
    avrlibc
    avrdude
  ];
}
