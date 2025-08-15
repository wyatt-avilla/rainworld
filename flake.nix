{
  description = "Plant remote management and automation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
    rust-overlay.url = "github:oxalica/rust-overlay";

    nix-checks = {
      url = "github:wyatt-avilla/nix-ci";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      esp-dev,
      rust-overlay,
      nix-checks,
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

        backendOutputs = import ./backend { inherit self pkgs system; };
        frontendOutputs = import ./frontend { inherit self pkgs system; };
      in
      {
        devShells = esp32Outputs.devShells // backendOutputs.devShells // frontendOutputs.devShells;
        packages = esp32Outputs.packages // backendOutputs.packages // frontendOutputs.packages;
        checks = {
          formatting = nix-checks.lib.mkFormattingCheck {
            inherit pkgs;
            src = self;
          };

          linting = nix-checks.lib.mkLintingCheck {
            inherit pkgs;
            src = self;
          };

          dead-code = nix-checks.lib.mkDeadCodeCheck {
            inherit pkgs;
            src = self;
          };
        };
        inherit (esp32Outputs) apps;
      }
    );
}
