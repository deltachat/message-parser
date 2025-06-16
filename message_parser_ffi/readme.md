```sh
cc -o ../target/debug/main -L ../target/debug -l message_parser_ffi smoke_test/main.c
../target/debug/main
```

```
cargo run --features headers --bin generate-headers
```