{
  description = "Plant remote management and automation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      esp-dev,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        esp32Outputs = import ./esp32 {
          inherit
            self
            pkgs
            system
            esp-dev
            ;
        };
      in
      {
        devShells = esp32Outputs.devShells;
        packages = esp32Outputs.packages;
        inherit (esp32Outputs) apps;
      }
    );
}
