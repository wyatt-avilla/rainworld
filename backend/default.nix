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

  influxPidFile = "influx.pid";

  influxStartScript = pkgs.writeShellScriptBin "start-influx" ''
    set -e

    INFLUX_DIR="$PWD/.dev-data/influxdb"
    INFLUX_PORT="8086"

    mkdir -p "$INFLUX_DIR"

    export INFLUXDB3_NODE_IDENTIFIER_PREFIX="rainworld_dev"
    export INFLUXDB3_HTTP_BIND_ADDR="0.0.0.0:$INFLUX_PORT"
    export INFLUXDB3_DB_DIR=$INFLUX_DIR/data
    export INFLUXDB3_OBJECT_STORE="file"

    echo "Starting InfluxDB on $INFLUXDB3_HTTP_BIND_ADDR ..."
    echo "Using directory: $INFLUX_DIR"

    ping_db() {
        ${pkgs.lib.getExe pkgs.wget} -qO- "$INFLUXDB3_HTTP_BIND_ADDR"/ping > /dev/null 2>&1
    }

    if ping_db; then
        echo "InfluxDB is already running"
        exit 0
    fi

    nohup ${pkgs.lib.getExe pkgs.influxdb3} serve > /dev/null 2>&1 &

    INFLUX_PID=$!
    echo "InfluxDB started with PID $INFLUX_PID"

    echo "Waiting for InfluxDB to be ready..."
    for i in {1..30}; do
      if ping_db; then
          echo "InfluxDB is ready!"
          break
      fi
      sleep 1
    done

    echo "InfluxDB is running at http://localhost:$INFLUX_PORT"
    echo "To stop: kill $INFLUX_PID"
    echo "PID saved to $INFLUX_DIR/${influxPidFile}"
    echo $INFLUX_PID > $INFLUX_DIR/${influxPidFile}
  '';

  influxStopScript = pkgs.writeShellScriptBin "stop-influx" ''
    INFLUX_DIR="$PWD/.dev-data/influxdb"
    PID_FILE=$INFLUX_DIR/${influxPidFile}

    if [[ -f $PID_FILE ]]; then
      PID=$(cat $PID_FILE)
      if kill -0 $PID 2>/dev/null; then
        echo "Stopping InfluxDB (PID: $PID)..."
        kill $PID
        rm -f $PID_FILE
        echo "InfluxDB stopped"
      else
        echo "InfluxDB process not running"
        rm -f $PID_FILE
      fi
    else
      echo "No PID file found"
    fi
  '';
in
{
  devShells.backend = pkgs.mkShell {
    name = "backend";
    nativeBuildInputs =
      nativeRustToolchain
      ++ (with pkgs; [
        rust-analyzer
        influxdb3

        influxStartScript
        influxStopScript
      ]);

    buildInputs = with pkgs; [
      pkg-config
      openssl
    ];
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
      meta.mainProgram = "rainworld-backend";
      buildAndTestSubdir = "backend";
      src = ../.;

      checkPhase = ''
        cargo clippy --package ${backend} --all-features -- -W clippy::pedantic -D warnings
        cargo fmt --package ${backend} --check
        cargo test --package ${backend}
      '';

      nativeBuildInputs =
        nativeRustToolchain
        ++ (with pkgs; [
          pkg-config
          openssl
        ]);

      buildInputs = with pkgs; [
        pkg-config
        openssl
      ];
    };
}
