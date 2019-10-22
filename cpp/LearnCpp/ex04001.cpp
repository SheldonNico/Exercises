#include <iostream>

int main() {
  int num1, num2;
  std::cout << "Enter an integer: ";
  std::cin >> num1;
  std::cout << "Enter a largeer integer: ";
  std::cin >> num2;

  if (num1 > num2) {
    int tmp = num1; // tmp created here
    num1 = num2;
    num2 = tmp;
  } // tmp dies here

  std::cout << "The smaller value is " << num1 << "\n";
  std::cout << "The larger value is " << num2 << "\n";
  return 0;
}
