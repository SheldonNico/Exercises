#include <iostream>
#include <functional>

void print(unsigned int value) {
  std::cout << value << "\n";
}
void print(float value) {
  std::cout << value << "\n";
}

void printValues(int x = 10, int y = 20, int z = 30) {
  std::cout << "values: x = " << x << " y = " << y << " z = " << z << "\n";
}

int foo() {
  return 5;
}



int main() {

  //print('a');
  print(3.14f);

  printValues(5, 3);

  std::cout << "function address " << reinterpret_cast<void*>(printValues) << "\n";

  //int (*foo_ptr)() = foo;
  auto* foo_ptr = foo;
  //void (*printValues_ptr) (int, int, int) = printValues;
  auto* printValues_ptr = printValues;

  std::cout << foo_ptr << printValues_ptr << "\n";
  (*printValues)(3, 2, 1);
  printValues(9, 9, 6);


  std::function<int()> foo_ptr_ = foo;
  std::function<void(int, int, int)> printValues_ptr_ = printValues_ptr_;

  return 0;
}
