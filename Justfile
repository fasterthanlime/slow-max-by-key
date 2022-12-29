run *args:
	#!/bin/bash -eux
	ulimit -s unlimited
	export RUSTFLAGS="-C link-args=-Wl,-zstack-size=134217728"
	cargo build --release {{args}}
	time ./target/release/day16
	time ./target/release/day16
	time ./target/release/day16