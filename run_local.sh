#!/usr/bin/env bash

# Parse command line arguments
SELECTED_BENCHMARKS="$1"
AVAILABLE_BENCHMARKS=("web" "rust" "profiled" "wasi" "nativecpu")

function printable_benchmarks() {
  echo "$(IFS=', '; echo "${AVAILABLE_BENCHMARKS[*]}")"
}
function help() {
  echo "Usage: $0 <benchmarks>"
  echo "  benchmarks: comma-separated list of benchmarks or 'all'"
  echo "  Available benchmarks: $(printable_benchmarks)"
}

if [ -z "$SELECTED_BENCHMARKS" ]; then
  help
  exit 1
fi

if [[ "$SELECTED_BENCHMARKS" == "all" ]]; then
  selected_all=true
else
  selected_all=false
  # Convert comma-separated list to array
  IFS=',' read -ra selected_array <<< "$SELECTED_BENCHMARKS"

  # Validate benchmarks
  for benchmark in "${selected_array[@]}"; do
    benchmark=$(echo "$benchmark" | xargs)  # trim whitespace
    if [[ "$benchmark" == "all" ]]; then
      echo "Error: 'all' cannot be combined with other benchmarks. Valid options: all (by itself), $(printable_benchmarks)"
      help
      exit 1
    fi

    # Check if benchmark is in available benchmarks
    if [[ ! " ${AVAILABLE_BENCHMARKS[@]} " =~ " ${benchmark} " ]]; then
      echo "Error: Invalid benchmark '$benchmark'. Valid options: all (by itself), $(printable_benchmarks)"
      help
      exit 1
    fi
  done
fi


function should_run_benchmark() {
  local benchmark="$1"
  if $selected_all; then
    return 0 # true
  fi
  if [[ " ${selected_array[*]} " == *" $benchmark "* ]]; then
    return 0 # true
  fi
  return 1 # false
}

if [ ! -d "results" ]; then
  mkdir results
fi
cd results || exit 1

# Perform web (chromium and firefox) benchmarks
if should_run_benchmark "web"; then
  echo "Running web benchmarks..."
  cd ..
  ./build_benchs.sh
  cd web_benchmark
  bun i
  node main.mjs
  mv benchmark_*.json ../results
  cd ../results
else
  echo "Skipping web benchmarks."
fi

# Perform rust native benchmarks
if should_run_benchmark "rust"; then
  echo "Running rust normal benchmark..."
  cd ../rust_benchs
  cargo run --release -- native
  mv benchmark_*.json ../results/
  cd ../results
else
  echo "Skipping rust benchmarks."
fi

if should_run_benchmark "profiled"; then
  echo "Running rust profiled benchmark..."

  cd ../rust_benchs
  # RUSTFLAGS='-C profile-generate' cargo run --release
  rm benchmark_*.json
  llvm-profdata merge -o merged.profdata default*.profraw 
  RUSTFLAGS="-C profile-use=$(pwd)/merged.profdata" cargo run --release -- nativeprof
  # cargo +nightly bench --manifest-path=../rust_benchs/Cargo.toml -- --no-capture
  mv benchmark_*.json ../results/

  cd ../results
fi

if should_run_benchmark "nativecpu"; then
  echo "Running rust profiled benchmark..."

  cd ../rust_benchs
  RUSTFLAGS='-C target-cpu=native' cargo run --release
  mv benchmark_*.json ../results/

  cd ../results
fi

# Perform rust wasi benchmarks
if should_run_benchmark "wasi"; then
  rustup target add wasm32-wasip2 --toolchain nightly 
  echo "Running rust wasi benchmark..."
  cd ../rust_benchs
  cargo build --release --target wasm32-wasip2
  # wasmtime --dir=.. --dir=. target/wasm32-wasip2/release/rust_benchs.wasm wasmtimeonone
  # mv benchmark_*.json ../results/
  TWO_POW_32=$((2**32))
  wasmtime -C compiler=cranelift -O signals-based-traps=y,memory-reservation=$TWO_POW_32,memory-guard-size=$TWO_POW_32 --dir=.. --dir=. target/wasm32-wasip2/release/rust_benchs.wasm wasmtime
  # wasmtime -C compiler=cranelift,inlining=y -O signals-based-traps=y,memory-reservation=$TWO_POW_32,memory-guard-size=$TWO_POW_32 --dir=.. --dir=. target/wasm32-wasip2/release/rust_benchs.wasm wasmtimeoall2
  mv benchmark_*.json ../results/
  cd ../results
else
  echo "Skipping rust benchmarks."
fi

