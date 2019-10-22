#include <iostream>
#include <cmath>
#include <algorithm>

int main() {
  double* data = nullptr;
  size_t size = pow(1014, 3) / 8; // this will produce 1GB
  for (int i=0; i < 5; i++) {
    data = new double[size];
    std::fill(data, data+size, 1.23);
    std::cout << "Iteration: " << i << " is done!" << std::endl;
  }
  delete[] data;
  int unused; std::cin >> unused;
  return 0;
}
