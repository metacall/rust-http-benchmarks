#!/bin/sh
DURATION=3
ACTIX_PORT=8080
WARP_PORT=8081
AXUM_PORT=8082

echo "Installing node modules..."
npm install

echo "Building binaries..."
cargo build --release

printf "benchmarking actix..."
./target/release/actix &
disown
sleep 3
wrk -t1 -c1 -d"$DURATION"s "http://127.0.0.1:$ACTIX_PORT" >>actix.bench &
echo "actix result saved to actix.bench"
printf "benchmarking warp..."
./target/release/warp &
disown
sleep 3
wrk -t1 -c1 -d"$DURATION"s "http://127.0.0.1:$WARP_PORT" >>warp.bench &
echo "warp result saved to warp.bench"

printf "benchmarking axum..."
./target/release/axum &
disown
sleep 3
wrk -t1 -c1 -d"$DURATION"s "http://127.0.0.1:$AXUM_PORT" >>axum.bench &
echo "axum result saved to axum.bench"

echo "killing http servers..."
sleep 6
pkill -c -9 actix
pkill -c -9 axum
pkill -c -9 warp
