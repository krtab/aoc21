Time runs:

```
cargo build --release --all-targets
hyperfine --warmup 3 './target/release/{n}' --parameter-list n "01,02,03,04,05,06"
```