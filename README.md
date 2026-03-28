# curl-sys `if_nametoindex` repro

This repository is a GitHub Actions reproduction harness for the Windows MSVC
link failure:

`LNK2019 unresolved external symbol __imp_if_nametoindex`

It contains two levels of reproduction:

- `minimal`: a tiny crate that pins `curl-sys = 0.4.86` and enables vendored
  `static-curl`
- `cargo-baseline` / `cargo-patched`: a closer repro that checks out Cargo at
  `8e799e11cd` and builds `cargo --features all-static` on `windows-2025`

The workflow runs on `windows-2025` and compares:

- the unmodified path
- a patched path that adds `cargo:rustc-link-lib=iphlpapi` inside
  `curl-sys/build.rs`

If the baseline Cargo build fails with `__imp_if_nametoindex` and the patched
build succeeds, that is strong evidence that the missing `Iphlpapi.lib` link is
the real cause.
