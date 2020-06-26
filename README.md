# Bench compilation time of nanoserde and serde

Benching scenario - read tiled tilemap from json and than serialize some binary data for networking messages. This is based on the real project migrated from serde to nanoserde.

Benches are just single runs on quite old machine(x230 with i5-3230M). 

If build-time dependencies optimisation level is not specified in test - they was in default state (no `profile.release.build-override` section in Cargo.toml).

## Debug build, json only
```
> cargo build --features "tiled-nanoserde"
..
Finished dev [unoptimized + debuginfo] target(s) in 3.90s

> cargo build --features "tiled-serde"
..
Finished dev [unoptimized + debuginfo] target(s) in 29.47s

Nanoserde: 4s   
Serde: 30s
```

## Release wasm build, json only

```
> cargo build --features "tiled-nanoserde" --release --target wasm32-unknown-unknown
..
Finished release [optimized] target(s) in 6.80s

> cargo build --features "tiled-serde" --release --target wasm32-unknown-unknown
..
Finished release [optimized] target(s) in 58.02s
```

Nanoserde: 7s   
Serde: 58s

## Wasm filesize, json only

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

## Debug build, json + bin

```
> cargo build --features "tiled-nanoserde, nano-bin"
Finished dev [unoptimized + debuginfo] target(s) in 4.10s

> cargo build --features "tiled-serde, serde-bin"
Finished dev [unoptimized + debuginfo] target(s) in 30.40s
```

nanoserde: 4s   
serde: 30s

## Release wasm build, json + bin, without  [profile.release.build-override] opt-level = 0

```
> cargo build --features "tiled-nanoserde, nano-bin" --target wasm32-unknown-unknown --release
Finished release [optimized] target(s) in 6.32s

> cargo build --features "tiled-serde, serde-bin" --target wasm32-unknown-unknown --release
Finished release [optimized] target(s) in 59.35s
```

nanoserde: 6s   
serde: 59s

## Release wasm build, json + bin, with  [profile.release.build-override] opt-level = 0

```
> cargo build --features "tiled-nanoserde, nano-bin" --target wasm32-unknown-unknown --release
Finished release [optimized] target(s) in 5.69s

> cargo build --features "tiled-serde, serde-bin" --target wasm32-unknown-unknown --release
Finished release [optimized] target(s) in 32.91s
```

nanoserde: 6s   
serde: 33s

## Wasm filesize, json+bin

```
# nanoserde
> wasm-strip target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
> target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
160K

# serde
> wasm-strip target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
> target/wasm32-unknown-unknown/release/nanoserde-bench.wasm
216K

```

## Incremental debug build time, json+bin

```
> cargo run --features "tiled-nanoserde, nano-bin"
Finished dev [unoptimized + debuginfo] target(s) in 0.36s

> cargo run --features "tiled-serde, serde-bin"
Finished dev [unoptimized + debuginfo] target(s) in 1.65s
```

nanoserde: 0.4s   
serde: 1.6s

