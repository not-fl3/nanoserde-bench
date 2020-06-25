## Debug build
```
> cargo build --features "tiled-nanoserde
..
Finished dev [unoptimized + debuginfo] target(s) in 3.90s

> cargo build --features "tiled-serde"
..
Finished dev [unoptimized + debuginfo] target(s) in 29.47s

Nanoserde: 4s
Serde: 30s

## Release wasm build
```
> cargo build --features "tiled-nanoserde" --release --target wasm32-unknown-unknown
..
Finished release [optimized] target(s) in 6.80s

> cargo build --features "tiled-nanoserde" --release --target wasm32-unknown-unknown
..
Finished release [optimized] target(s) in 58.02s
```

Nanoserde: 7s
Serde: 58s

## Wasm filesize

```
# nanoserde
> wasm-strip target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
> du -sh target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
156K

# serde
> wasm-strip target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
> du -sh target/wasm32-unknown-unknown/release/nanoserde-bench.wasm

212K
```

