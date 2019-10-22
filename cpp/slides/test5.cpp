#include <stdio.h>
int* generateData(const int& size) {
  int* out = new int[size];
  return out;
}

void UseDataForGood(const int* const data, int size) {
  // do nothing
}

void UseDataForBad(const int* const data, int size) {
  delete [] data;
}

int main() {
  int size = 10;
  int* data = generateData(size);
  UseDataForGood(data, size);
  UseDataForBad(data, size);
  delete [] data;

  return 0;
}
