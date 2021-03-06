let
  pkgs = import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/d105075a1fd870b1d1617a6008cb38b443e65433.tar.gz";
    sha256 = "1jcs44wn0s6mlf2jps25bvcai1rij9b2dil6zcj8zqwf2i8xmqjh";
  }) {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      gcc
      pkgconfig
      openssl.dev
      zlib.dev
      rustc
      cargo
      SDL2
    ];
  }
