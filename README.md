# RustCxxCMake

[![build](https://github.com/jon-smith/RustCxxCmake/actions/workflows/build.yaml/badge.svg)](https://github.com/jon-smith/RustCxxCmake/actions/workflows/build.yaml)

Calling Rust code from C++ using [Cxx](https://github.com/dtolnay/cxx) and build using CMake and [Corrosion](https://github.com/AndrewGaspar/corrosion).

Rust code to be exposed to C++ can be found in `src/rust_lib/src/lib.rs`, and C++ calling code in `src/main.cpp`.

# Build/Run

CMake presets are available for gcc, msvc x64 and clang, in both release and debug configurations.

```sh
preset=gcc-release
cmake --preset ${preset}
cmake --build out/build/${preset}
out/build/${preset}/src/rust_cxx_cmake
```

Tested locally on MacOS with `GCC 14.1.0 aarch64-apple-darwin23` and `Clang 15.0.0 arm64-apple-darwin23.5.0`.

CI builds are defined in [build.yaml](./.github/workflows/build.yaml).
