{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix-rust = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix-rust }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { 
      inherit system;
      overlays = [ fenix-rust.overlays.default ];
    };
    rustPkg = pkgs.fenix.stable.withComponents [
     "cargo"
     "clippy"
     "rust-src"
     "rustc"
     "rustfmt"
    ];
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustPkg
        rust-analyzer-nightly
        llvmPackages.bintools
        pkg-config
        openssl
        cargo-watch
        cargo-tarpaulin
        cargo-audit
        cargo-udeps
        clang
        sqlx-cli
        postgresql
        flyctl
      ];
    };
  };
}
