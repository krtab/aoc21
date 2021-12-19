Time runs:

```
hyperfine --warmup 3 './target/release/{n}' --parameter-list n "01,02,03"
```