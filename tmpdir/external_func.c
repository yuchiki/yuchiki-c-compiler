#include <stdio.h>

long external_func(int a, int b, int c, int d, int e, int f) {
  printf("hello! %d\n", a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f);
  return 5;
}
