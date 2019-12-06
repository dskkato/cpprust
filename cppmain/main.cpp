#include "rustlib.h"
#include <iostream>

int main() {
  const Point p = {3, 3};
  const int32_t v = hello(p);
  
  std::cout << "Hello, C++" << v << std::endl;
  return 0;
}
