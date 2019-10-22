#include <iostream>
int main() {
  int size = 5;
  int* ptr_1 = new int[size];
  int* ptr_2 = ptr_1;

  ptr_1[0] = 100;
  ptr_1[1] = 80;
  std::cout << "1: " << ptr_1 << " 2: " << ptr_2 << std::endl;
  std::cout << "ptr_2[0]: " << ptr_2[0] << " " << ptr_2[1] << std::endl;
  delete[] ptr_1;
  ptr_1 = nullptr;

  std::cout << "1: " << ptr_1 << " 2: " << ptr_2 << std::endl;
  std::cout << "ptr_2[0]: " << ptr_2[0] << " " << ptr_2[1] << std::endl;
}
