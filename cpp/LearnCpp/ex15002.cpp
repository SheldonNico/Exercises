#include <iostream>
#include <cassert>

void func(const int& lref) {
  std::cout << "l-value reference to const\n";
}
void func(int&& rref) {
  std::cout << "r-value reference\n";
}
void func(int& lref) {
  std::cout << "l-value reference\n";
}

int main() {
  int&& rref = 5;
  rref = 4;
  int x = 5;
  int& lref = x;
  std::cout << rref << " " << lref << "\n";

  func(x);
  func(5);

  return 0;
}
