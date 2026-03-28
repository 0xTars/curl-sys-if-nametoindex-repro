# curl-sys `if_nametoindex` repro

This repository is a minimal reproduction for the Windows MSVC link failure:

`LNK2019 unresolved external symbol __imp_if_nametoindex`

The repro pins `curl-sys = 0.4.86+curl-8.19.0` and enables vendored
`static-curl` so that GitHub Actions uses the same general build path that
surfaced in Rust CI.

The workflow has two jobs on `windows-2025`:

- `baseline`: builds the unmodified crate and is expected to fail while linking
  vendored `curl-sys`
- `patched`: patches `curl-sys` in the local Cargo registry to add
  `cargo:rustc-link-lib=iphlpapi`, then rebuilds

If `baseline` fails with `__imp_if_nametoindex` and `patched` succeeds, that is
strong evidence that the missing `Iphlpapi.lib` link is the real cause.
