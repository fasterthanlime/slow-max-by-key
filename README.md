# slow-max-by-key

Reproduction to show that a some Rust code gets slower when refactored to use
`Iterator::max_by_key`.

## Sample results

(Note that the nightly version is pinned, see `rust-toolchain.toml`)

```shell
$ rustc -vV
rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: 270c94e484e19764a2832ef918c95224eb3f17c7
commit-date: 2022-12-28
host: x86_64-unknown-linux-gnu
release: 1.68.0-nightly
LLVM version: 15.0.6

$ cargo bench
    Finished bench [optimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/release/deps/day16-95df0454694c3cc4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-27b51c49825687a1)
manual                  time:   [3.0159 ms 3.0281 ms 3.0424 ms]
                        change: [-0.9012% -0.3251% +0.2743%] (p = 0.28 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

max_by_key              time:   [6.0376 ms 6.0652 ms 6.0923 ms]
                        change: [-1.9902% -1.4124% -0.8517%] (p = 0.00 < 0.05)
                        Change within noise threshold.
```