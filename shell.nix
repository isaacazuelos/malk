let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

  avrlibc = nixpkgs.pkgsCross.avr.buildPackages.libcCross;

in nixpkgs.mkShell {
  name = "malk";

  buildInputs = with nixpkgs; [
    # When updating, check availablility of extensions at
    # https://rust-lang.github.io/rustup-components-history
    ((rustChannelOf {
      date = "2020-08-27";
      channel = "nightly";
    }).rust.override {
      extensions = [ "rust-src" "rustfmt-preview" ];
      targetExtensions = [ "rust-src" ];
    })
    pkgsCross.avr.buildPackages.gcc8
    avrlibc
    avrdude
  ];
}
