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
    export INFLUXD_HTTP_BIND_ADDRESS=:$INFLUX_PORT
    export INFLUXD_ENGINE_PATH=:$INFLUX_DIR/engine
    export INFLUXD_CONFIG_PATH=:$INFLUX_DIR/config

    export INFLUXDB_DATA_DIR=:$INFLUX_DIR/data
    export INFLUXDB_DATA_WAL_DIR=:$INFLUX_DIR/wal
    export INFLUXDB_META_DIR=:$INFLUX_DIR/meta

    echo "Starting InfluxDB on port $INFLUX_PORT..."
    echo "Using directory: $INFLUX_DIR"

    if ${pkgs.influxdb}/bin/influx ping > /dev/null 2>&1; then
        echo "InfluxDB is already running!"
        exit 0
    fi

    nohup ${pkgs.influxdb}/bin/influxd > /dev/null 2>&1 &

    INFLUX_PID=$!
    echo "InfluxDB started with PID $INFLUX_PID"

    echo "Waiting for InfluxDB to be ready..."
    for i in {1..30}; do
    if ${pkgs.influxdb}/bin/influx ping > /dev/null 2>&1; then
        echo "InfluxDB is ready!"

        ${pkgs.influxdb}/bin/influx -execute "CREATE DATABASE plant_monitoring" 2>/dev/null || true
        echo "Database 'plant_monitoring' ready"

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
        influxdb2
        influxdb2-cli

        influxStartScript
        influxStopScript
      ]);
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
