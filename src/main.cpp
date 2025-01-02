#include "cpp_rust_bridge/lib.h"
#include <iostream>
#include <format>
#include <numbers>

int main()
{
  // Primitive type input and result
  std::cout << "2 x 2 = " << rust_cxx_square(2) << "\n";

  // Create opaque rust structure
  const auto message = rust_cxx_build_message_container("hello");

  // Use opaque structure in rust function
  rust_cxx_print_message(message);

  // String input and result
  std::cout << "wow = " << std::string(rust_cxx_wow("cpp")) << "\n";

  // Shared structure input and result
  const auto rotationResult = rust_cxx_rotate(XY{.x = 0, .y = 1}, std::numbers::pi);
  std::cout << std::format("Rotation result: {},{}\n", rotationResult.x, rotationResult.y);

  // Shared composite structure
  const auto compositeStruct = rust_cxx_build_composite(XY{.x = 0, .y = 1}, std::numbers::pi);
  std::cout << std::format("Composite struct values: {},{},{}\n", compositeStruct.point.x, compositeStruct.point.y, compositeStruct.value);

  // Function that relies on system libraries
  const auto getResponse = std::string(rust_cxx_http_get("https://httpbin.org/get", ""));
  std::cout << std::format("GET: {}\n", getResponse);

  return 0;
}
