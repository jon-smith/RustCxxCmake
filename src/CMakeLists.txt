corrosion_import_crate(MANIFEST_PATH rust_lib/Cargo.toml)
corrosion_add_cxxbridge(cpp_rust_bridge CRATE rust_lib MANIFEST_PATH rust_lib FILES lib.rs)

add_executable(rust_cxx_cmake main.cpp)
target_link_libraries(rust_cxx_cmake PRIVATE cpp_rust_bridge)
