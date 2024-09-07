#include "lib.rs.h"
#include <iostream>
#include <format>
#include <numbers>

int main()
{
  // Primitive type input and result
  std::cout << "2 x 2 = " << rust_cxx_square(2) << "\n";

  // String input and result
  std::cout << "wow = " << std::string(rust_cxx_wow("cpp")) << "\n";

  // Shared structure input and result
  const auto rotationResult = rust_cxx_rotate(XY{.x = 0, .y = 1}, std::numbers::pi);
  std::cout << std::format("Rotation result: {},{}\n", rotationResult.x, rotationResult.y);

  // Function that relies on system libraries
  const auto getResponse = std::string(rust_cxx_http_get("https://httpbin.org/get", ""));
  std::cout << std::format("GET: {}\n", getResponse);

  return 0;
}
