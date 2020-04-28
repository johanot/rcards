let
  pkgs = import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs-channels/archive/708cb6b307b04ad862cc50de792e57e7a4a8bb5a.tar.gz";
    sha256 = "0fjwv9sxl3j6z0jszaznvz891mn44fz6lqxsa2fkx9xi5mkz63jm";
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
