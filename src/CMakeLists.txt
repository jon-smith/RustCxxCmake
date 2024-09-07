set(RUST_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib)
set(RUST_LIB_SOURCE_FILE ${RUST_LIB_DIR}/src/lib.rs)

set(CARGO_MANIFEST ${RUST_LIB_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_BINARY_DIR}/target)

if(CMAKE_BUILD_TYPE STREQUAL Debug)
    set(CARGO_PROFILE dev)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/debug)
elseif(CMAKE_BUILD_TYPE STREQUAL RelWithDebInfo)
    set(CARGO_PROFILE release-with-debug)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/release-with-debug)
else()
    set(CARGO_PROFILE release)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/release)
endif()

set(RUST_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/rust_lib/src/lib.rs.cc)
set(RUST_LIB ${CARGO_BUILD_FOLDER}/${CMAKE_STATIC_LIBRARY_PREFIX}rust_lib${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
    OUTPUT ${RUST_BRIDGE_CPP} ${RUST_LIB}
    COMMAND cargo build --profile ${CARGO_PROFILE} --manifest-path ${CARGO_MANIFEST} --target-dir ${CARGO_TARGET_DIR}
    DEPENDS ${RUST_LIB_SOURCE_FILE} ${CARGO_MANIFEST}
    USES_TERMINAL
    COMMENT "Running cargo..."
)

add_executable(rust_cxx_cmake main.cpp ${RUST_BRIDGE_CPP})

target_include_directories(
    rust_cxx_cmake
    PRIVATE
    include/
    ${CARGO_TARGET_DIR}/cxxbridge/rust_lib/src/
)

target_link_libraries(rust_cxx_cmake PUBLIC ${RUST_LIB})

set_target_properties(
    rust_cxx_cmake
    PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

# Link system libraries for TLS used by reqwest library

if (APPLE)

    find_library(SystemConfiguration SystemConfiguration)
    if (NOT SystemConfiguration)
        message(FATAL_ERROR "SystemConfiguration not found")
    endif()
    target_link_libraries(rust_cxx_cmake PRIVATE ${SystemConfiguration})

    find_library(SecurityFramework Security)
    if (NOT SecurityFramework)
        message(FATAL_ERROR "SecurityFramework not found")
    endif()
    target_link_libraries(rust_cxx_cmake PRIVATE ${SecurityFramework})

    find_library(CoreFoundation CoreFoundation)
    if (NOT CoreFoundation)
        message(FATAL_ERROR "CoreFoundation not found")
    endif()
    target_link_libraries(rust_cxx_cmake PRIVATE ${CoreFoundation})

elseif (WIN32)

    target_link_libraries(
        rust_cxx_cmake 
        PRIVATE 
        advapi32 crypt32 ws2_32 bcrypt Secur32 Ncrypt userenv)

elseif (LINUX)

    find_package(OpenSSL REQUIRED)
    target_link_libraries(
        rust_cxx_cmake 
        PRIVATE 
        OpenSSL::SSL OpenSSL::Crypto)

endif()

# Additional properties for windows builds

if (WIN32)

    set_target_properties(
        rust_cxx_cmake
        PROPERTIES
        MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
        RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
        RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )

endif()
