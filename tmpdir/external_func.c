int external_func(int a, int b, int c, int d, int e, int f) {
  return a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f;
}

int *test_malloc_4() {
  long *a = (int *)malloc(4 * sizeof(int));
  a[0] = 1;
  a[1] = 2;
  a[2] = 3;
  a[3] = 4;
  return a;
}
