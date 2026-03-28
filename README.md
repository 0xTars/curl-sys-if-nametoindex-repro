# curl-sys `if_nametoindex` repro

This repository is a GitHub Actions reproduction harness for the Windows MSVC
link failure:

`LNK2019 unresolved external symbol __imp_if_nametoindex`

The current workflow is intentionally shaped like the real Rust CI path instead
of a plain `cargo build`.

It checks out `rust-lang/rust` at
`f0ff0628bc708f087e33d57efe406b8cff323a1f`, initializes submodules, configures
the tree with the same key knobs as the failing `x86_64-msvc-ext1` job, and
runs:

`python x.py --stage 2 test src/tools/cargo`

Two variants are compared on public `windows-2025` runners:

- `xpy-baseline`: unmodified path
- `xpy-patched`: same path, but patches downloaded
  `curl-sys 0.4.86+curl-8.19.0/build.rs` to add
  `cargo:rustc-link-lib=iphlpapi`

If the baseline `x.py` build hits `__imp_if_nametoindex` and the patched build
stops doing so, that is strong end-to-end evidence that the missing
`Iphlpapi.lib` link is the real cause.
