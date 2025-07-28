{ pkgs, ... }:
let
  nativeRustToolchain = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = [
        "clippy"
        "rust-src"
      ];
    })
  ];
in
{
  devShells.backend = pkgs.mkShell {
    name = "backend";
    nativeBuildInputs = nativeRustToolchain ++ [ pkgs.rust-analyzer ];
  };

  packages.backend =
    let
      backend = "backend";
    in
    pkgs.rustPlatform.buildRustPackage {
      name = backend;
      pname = backend;
      cargoLock = {
        lockFile = ../Cargo.lock;
      };
      buildAndTestSubdir = "backend";
      src = ../.;

      checkPhase = ''
        cargo clippy --package ${backend} --all-features -- -W clippy::pedantic -D warnings
        cargo fmt --package ${backend} --check
      '';

      nativeBuildInputs = nativeRustToolchain;
      meta.mainProgram = "rainworld-backend";
    };
}
