#include <iostream>
#include <string>
#include <functional>

//using arithmeticFcn = std::function<int(const int&, const int&)>; // not work?
using arithmeticFcn = int (*)(const int&, const int&);

int getInt() {
  int num{0};
  do {
    std::cin.clear();
    std::cout << "Please enter an integer: ";
    std::cin >> num;
    std::cin.ignore(32767, '\n');
  } while (std::cin.fail());

  return num;
}

char getOp() {
  std::string ops { "+-*/" };
  char op {' '};
  do {
    std::cin.clear();
    std::cout << "Please enter the operator (+-*/): ";
    std::cin >> op;
    std::cin.ignore(32767, '\n');
  } while (std::cin.fail() || (ops.find(op) == std::string::npos) );

  return op;
}

int add(const int& num1, const int& num2) {
  return num1 + num2;
}

int subtract(const int& num1, const int& num2) {
  return num1 - num2;
}

int multiply(const int& num1, const int& num2) {
  return num1 * num2;
}

int divide(const int& num1, const int& num2) {
  return num1 / num2;
}

arithmeticFcn getArithmeticFunction(const char& op) {
  switch (op) {
    default: // notice default is here
    case ('+'): return add;
    case ('-'): return subtract;
    case ('*'): return multiply;
    case ('/'): return divide;
  }
}

int main() {
  int num1 = getInt();
  int num2 = getInt();
  char op = getOp();

  std::cout << num1 << op << num2 << " = " << getArithmeticFunction(op)(num1, num2) << "\n";
}
