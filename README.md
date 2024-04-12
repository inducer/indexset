# A mini prototype ordered set

A set implementation for Python that remembers insertion order, just like
Python's built-in `dict`, obtained by wrapping the Rust 
[indexmap](https://github.com/indexmap-rs/indexmap) crate into Python via
[PyO3](https://pyo3.rs).

You'll need to have a rust toolchain, see [rustup](https://rustup.rs/).

To build:
```
pipx install maturin
maturin develop
```

To test:
```
python -m pytest
```
