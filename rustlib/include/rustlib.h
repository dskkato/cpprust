#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Point {
  int32_t _0;
  uintptr_t _1;
};

extern "C" {

int32_t hello(Point p);

} // extern "C"
