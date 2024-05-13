#!/bin/sh

PORT=8080
DURATION=3

echo "Installing node modules..."
npm install

echo "Building binaries..."
cargo build --release

export LD_LIBRARY_PATH="/usr/local/lib"

benchmark_framework() {
    # Don't output the dummy run
    if [ $2 ]; then
        echo "benchmarking $1..."
    fi

    # Run the server
    if [ $2 ]; then
        "./target/release/$1" &
    else
        # Don't output the dummy run
        "./target/release/$1" > /dev/null 2>&1 &
    fi
    # Wait for it bootstrap
    sleep 3

    # Benchmark and store the results if the second argument is true
    if [ $2 ]; then
        wrk -t1 -c1 -d"$DURATION"s "http://127.0.0.1:$PORT" >> "$1.bench"
    else
        # Don't output the dummy run
        wrk -t1 -c1 -d"$DURATION"s "http://127.0.0.1:$PORT" > /dev/null 2>&1
    fi

    # Kill all and make sure everything is clear
    disown -a
    pkill -c -9 wrk > /dev/null 2>&1
    pkill -c -9 $1 > /dev/null 2>&1
    kill $(lsof -t -i:$PORT) > /dev/null 2>&1
    sleep 1

    if [ $2 ]; then
        # Don't output the dummy run
        echo "$1 result saved to $1.bench"
    fi
}

# The first to be ran is always is the slowest. Let's run one as a dummy.
benchmark_framework actix

benchmark_framework axum true
benchmark_framework warp true
benchmark_framework actix true
