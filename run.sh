#!/bin/bash

cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp

./target/release/tcp &
pid=$!
sudo ip addr add dev tun0 10.10.10.1/24
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid

