#include "rustlib.h"
#include <iostream>

int main() {
  const int32_t v = hello(2);
  assert(v == -1);
  std::cout << "Hello, C++" << v << std::endl;
  return 0;
}
