let
  pkgs = import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs-channels/archive/2faa76db27c4a8045d050d9b390dbb2249b0f3c0.tar.gz";
    sha256 = "19g7bas7kd4cipbwxh1hw0kgh4670plsfmk5gqsnivq0x12lbhqd";
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
