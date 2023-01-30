# is-zeroed-comparisons
```
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/is-zeroed-comparisons`
4000 byte by byte: true, 196.208µs
4000 in 4 byte chunks: true, 123.625µs
4000 in 8 byte chunks: true, 61.833µs
4000 in 16 byte chunks: true, 30.916µs
4000 as u128 chunks: true, 37.875µs
4000 in 1024 chunks: true, 1.458µs
```