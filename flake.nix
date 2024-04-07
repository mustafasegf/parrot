{
  description = "flake for parrot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; rec {
        devShells.default = mkShell rec {

          nativeBuildInputs = [ cmake pkg-config ];
          buildInputs = [
            openssl
            libtool
            automake
            autoconf
            ffmpeg_5-full
            opusfile
            rust-bin.stable.latest.complete
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${lib.makeLibraryPath buildInputs}
          '';

        };
      });
}
