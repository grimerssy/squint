{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [ (import rust-overlay) ];
      in
      {
        devShells.default = pkgs.mkShellNoCC {
          packages = with pkgs; [
            (rust-bin.stable."1.72.0".default.override {
              extensions = [ "rust-src" "clippy" "rust-analyzer" ];
            })
            cargo-hack
          ];
        };
      }
    );
}
