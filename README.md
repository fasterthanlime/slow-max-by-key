# slow-max-by-key

Reproduction to show that a some Rust code gets slower when refactored to use
`Iterator::max_by_key`.

## Fast version

```shell
$ just run --features production,quiet
    Finished release [optimized + debuginfo] target(s) in 0.00s
+ ./target/release/day16
final pressure = 1947

real    0m0,158s
user    0m0,153s
sys     0m0,005s
+ ./target/release/day16
final pressure = 1947

real    0m0,158s
user    0m0,151s
sys     0m0,008s
+ ./target/release/day16
final pressure = 1947

real    0m0,155s
user    0m0,147s
sys     0m0,008
```

## Slow version

```shell
$ just run --features production,quiet,max_by_key
+ ./target/release/day16
final pressure = 1947

real    0m0,815s
user    0m0,815s
sys     0m0,000s
+ ./target/release/day16
final pressure = 1947

real    0m0,825s
user    0m0,815s
sys     0m0,010s
+ ./target/release/day16
final pressure = 1947

real    0m0,847s
user    0m0,842s
sys     0m0,005s
```