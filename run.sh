#!/bin/bash
cargo b --release
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
./target/release/rust-http-server &
pid=$!
trap "kill $pid" INT TERM
wait $pid
