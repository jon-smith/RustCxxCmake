#include "cpp_rust_bridge/lib.h"
#include <iostream>

int main() {
  std::cout << "2 x 2 = " << rust_cxx_square(2) << "\n";
  return 0;
}
