## Onload rust bindings

> Notice: This project doesn't support windows

### Installation

```toml
onload = { version = "0.3", features = "9", default-features = false }
# 9 for readhat 9.5+   9.0.1.86 
# 8 for lower version  8.x
```

### usage

```rust 
use onload::binding::*;
```

### How to update locally

##### Required

- Install `openonload` and `openopen-devel (only for 9.5+)`
- Install `tcpdirect`

##### Steps

1. `git clone https://github.com/HFQR/onload`
2. `cd onload`
3. `cargo build --features=debug`
4. `cp ./target/debug/build/onload-cc74b2c38d00e273/output/bindings.rs ./src/`

notice:  in my case, dir is under `onload-cc74b2c38d00e273`,

now you can locally export the bindings
