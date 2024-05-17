cmake_minimum_required(VERSION 3.23)

project(RustCxxCMake CXX)

include(FetchContent)

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
set(CMAKE_CXX_STANDARD 20)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

add_subdirectory(src)
