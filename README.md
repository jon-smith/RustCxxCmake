# RustCxxCMake

[![build](https://github.com/jon-smith/RustCxxCmake/actions/workflows/build.yaml/badge.svg)](https://github.com/jon-smith/RustCxxCmake/actions/workflows/build.yaml)

Calling Rust code from C++ using [Cxx](https://github.com/dtolnay/cxx) and build using CMake

## Code

Rust code to be exposed to c++ can be found in `src/rust_lib/src/lib.rs`, and c++ calling code in `src/main.cpp`.

We expose the following:

- `rust_cxx_square`: Simple function taking and returning a primitive

- `rust_cxx_build_message_container`: Function creating and returning an opaque rust structure

- `rust_cxx_print_message`: Function taking in and acting on an opaque rust structure

- `rust_cxx_rotate`: Using a shared structure `XY`, available to both rust and c++

- `rust_cxx_build_composite`: Using a shared structure `CompositeStructure`, containing another shared structure `XY` as a member, available to both rust and c++

- `rust_cxx_wow`: Interfacing between c++ and rust strings

- `rust_cxx_http_get`: Calling rust library dependencies, relying on external system libraries for SSL (added to [CMakeLists.txt](./src/CMakeLists.txt))

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

# Prior Art

Have taken ideas/inspiration from the following projects:
https://github.com/paandahl/cpp-with-rust
https://github.com/trondhe/rusty_cmake
