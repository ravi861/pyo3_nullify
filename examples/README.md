The default "cargo build" will nullify all usages of pyo3 macros. Below command builds with pyo3 linking.
```
cargo build --features python-module
```