Works:

```
cargo test no_panic -- --nocapture
cargo test no_panic_str -- --nocapture
```

Broken:

```
cargo test panic_static_to_string -- --nocapture
cargo test panic_const_to_string -- --nocapture
cargo test panic_tostring -- --nocapture
cargo test panic_cstring -- --nocapture
cargo test panic_println -- --nocapture
cargo test panic_println -- --nocapture
```