#!/bin/bash

cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp

# start process in background
./target/release/tcp &
# get pid
pid=$!
sudo ip addr add dev tun0 10.10.10.1/24
sudo ip link set up dev tun0
# kill pid on INT or TERM
trap "kill $pid" INT TERM
wait $pid

