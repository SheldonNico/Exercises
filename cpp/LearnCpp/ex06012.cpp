#include <utility>
#include <iostream>

void swap(int& v1, int& v2) {
  //int tmp = v1;
  //v1 = v2;
  //v2 = tmp;
  v1 += v2;
  v2 = v1 - v2;
  v1 = v1 - v2;
}

int main() {
  int a = 10, b = 20;
  std::cout << "before swap a = " << a << ", b = " << b << "\n";
  swap(a, b);
  std::cout << "after swap a = " << a << ", b = " << b << "\n";

  return 0;
}
