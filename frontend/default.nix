{ pkgs, ... }:
let
  wasmTarget = "wasm32-unknown-unknown";

  nativeRustToolchain = with pkgs; [
    (rust-bin.nightly.latest.default.override {
      extensions = [
        "clippy"
        "rust-src"
      ];
      targets = [ wasmTarget ];
    })
  ];
in
{
  devShells.frontend = pkgs.mkShell {
    name = "frontend";
    nativeBuildInputs = nativeRustToolchain ++ (with pkgs; [ rust-analyzer ]);

    buildInputs = with pkgs; [ ];

    shellHook = ''
      export CARGO_BUILD_TARGET="${wasmTarget}"
    '';
  };

  packages.frontend =
    let
      frontend = "frontend";
    in
    pkgs.rustPlatform.buildRustPackage {
      name = frontend;
      pname = frontend;
      cargoLock = {
        lockFile = ../Cargo.lock;
      };
      buildAndTestSubdir = "frontend";
      src = ../.;

      buildPhase = ''
        cargo build -j $(nproc) -p ${frontend} --offline --release --target=${wasmTarget}
        mv target/stylers target/stylers-release
      '';

      checkPhase = ''
        # TODO: wasm-validate?
        cargo clippy --package ${frontend} --all-features -- -W clippy::pedantic -D warnings
        cargo fmt --package ${frontend} --check
      '';

      installPhase = ''
        mkdir -p $out/pkg

        cp target/${wasmTarget}/release/${frontend}.wasm $out/pkg/

        ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
        target/${wasmTarget}/release/${frontend}.wasm \
        --out-dir $out \
        --target web \
        --no-typescript

        ${pkgs.binaryen}/bin/wasm-opt \
        $out/${frontend}_bg.wasm \
        -o $out/${frontend}_bg.wasm \
        -Oz

        cp target/stylers-release/main.css $out/

        cat > $out/index.html << 'EOF'
        <!DOCTYPE html>
        <html>
        <head>
          <meta charset="utf-8">
          <title>Leptos App</title>
          <link rel="modulepreload" href="/${frontend}.js">
          <link rel="stylesheet" href="/main.css">
        </head>
        <body>
          <script type="module">
            import init, { hydrate } from './${frontend}.js';
            init().then(hydrate);
          </script>
        </body>
        </html>
      '';

      nativeBuildInputs = nativeRustToolchain ++ (with pkgs; [ ]);

      buildInputs = with pkgs; [ ];
    };
}
