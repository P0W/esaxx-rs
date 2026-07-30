// MSVC dynamic-CRT compatibility fix (upstream PR #19 / huggingface/tokenizers#1914).
// Two minimal changes from the crates.io 0.1.10 source, both MSVC-only:
//   1. Drop the hard-coded `.static_crt(true)`. Upstream forced the static MSVC CRT (`/MT`,
//      libcmt/libcpmt) unconditionally. When esaxx-rs is pulled in transitively (tokenizers'
//      `esaxx_fast` feature) into a binary that also links Rust std + native libs built
//      against the dynamic CRT (`/MD`, e.g. ONNX Runtime), mixing /MT and /MD is a hard MSVC
//      link error (LNK2005 + LNK1169 / LNK2038). Removing the override lets `cc` honor the
//      target's `crt-static` feature: `/MT` for `+crt-static`, `/MD` otherwise. That matches
//      standard dynamic-CRT Rust MSVC builds (the common case, incl. x86_64 + aarch64) while
//      still supporting fully-static consumers.
//   2. `.flag("-std=c++11")` -> `.flag_if_supported(...)` so MSVC cl.exe silently skips the
//      GCC/Clang-only flag instead of emitting D9002; no effect on GCC/Clang.

#[cfg(feature = "cpp")]
#[cfg(not(target_os = "macos"))]
fn main() {
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(feature = "cpp")]
#[cfg(target_os = "macos")]
fn main() {
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .flag("-stdlib=libc++")
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
