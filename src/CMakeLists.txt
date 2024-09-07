corrosion_import_crate(MANIFEST_PATH rust_lib/Cargo.toml)
corrosion_add_cxxbridge(cpp_rust_bridge CRATE rust_lib MANIFEST_PATH rust_lib FILES lib.rs)

add_executable(rust_cxx_cmake main.cpp)
target_link_libraries(rust_cxx_cmake PRIVATE cpp_rust_bridge)

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
        "advapi32" "crypt32" "ws2_32" "bcrypt" "Secur32" "Ncrypt")

elseif (LINUX)

    find_package(OpenSSL REQUIRED)
    target_link_libraries(
        rust_cxx_cmake 
        PRIVATE 
        OpenSSL::SSL OpenSSL::Crypto)

endif()