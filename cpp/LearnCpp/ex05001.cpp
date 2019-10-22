#include <cstdlib>
#include <iostream>
#include <exception>

int main() {
  int num1 = 0 , num2 = 0;
  char op = ' ';
  std::cin >> num1 >> op >> num2;

  int result = 0;
  switch (op) {
    case '+':
      result = num1 + num2;
      break;
    case '-':
      result = num1 - num2;
      break;
    case '/':
      result = num1 / num2;
      break;
    case '%':
      result = num1 % num2;
      break;
    default:
      std::cout << "not a valid operator\n";
      return 1;
  }

  std::cout << num1 << op << num2 << " = " << result << "\n";
  return 0;
}
