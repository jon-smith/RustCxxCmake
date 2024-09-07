cmake_minimum_required(VERSION 3.23)

project(RustCxxCMake CXX)

find_program(CARGO cargo PATHS "$ENV{HOME}/.cargo/bin/")
if (CARGO STREQUAL "CARGO-NOTFOUND")
    message(FATAL_ERROR "Requires cargo available in path, install via rustup https://rustup.rs/")
endif()

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
set(CMAKE_CXX_STANDARD 20)

add_subdirectory(src)
