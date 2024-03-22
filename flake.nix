{
  description = "Rust development environment";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  # Flake outputs
  outputs = { self, nixpkgs, rust-overlay }:
  let
    overlays = [
      (import rust-overlay)
    ];
    system = "x86_64-linux";
    pkgs = import nixpkgs { 
      inherit overlays system; 
    };
  in
  with pkgs;
  {
    devShells.${system}.default = mkShell {
      buildInputs = [
        rust-bin.stable.latest.default
        rust-bin.stable.latest.rust-src
        rust-analyzer
        llvmPackages.bintools
        pkg-config
        openssl
        cargo-watch
        cargo-tarpaulin
        cargo-audit
        clang
        sqlx-cli
        postgresql
      ];
      RUST_SRC_PATH="${rust-bin.stable.latest.rust-src}/lib/rustlib/src/rust/library/";
    };
  };
}
