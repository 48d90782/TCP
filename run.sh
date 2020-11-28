#!/bin/sh
cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp

./target/release/tcp &
pid=$!
sudo ip addr add dev tun0 10.107.1.3/24
sudo ip link set up dev tun0
wait $pid

