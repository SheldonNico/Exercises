#include <iostream> // 预处理指令

int main() {
  std::cout << "Enter an integer: ";
  int a{ 0 }, b{ 0 };
  std::cin >> a;
  std::cout << "Enter another integer: ";
  std::cin >> b;
  std::cout << a <<  " + " << b << " is " << a+b << "\n";
  std::cout << a <<  " - " << b << " is " << a-b << "\n";
  return 0;
}
