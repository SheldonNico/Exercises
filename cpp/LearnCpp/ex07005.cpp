#include <iostream>
#include <string>
#include <functional>
#include <array>

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

struct arithmeticStruct {
  char op;
  arithmeticFcn func;
};

//arithmeticStruct arithmeticArray[] {
  //{'+', add}, {'-', subtract}, {'*', multiply}, {'/', divide},
//};
static const std::array<arithmeticStruct, 4> arithmeticArray { {
  {'+', add}, {'-', subtract}, {'*', multiply}, {'/', divide},
} };

arithmeticFcn  getArithmeticFunction(const char& op) {
  for (auto as : arithmeticArray) {
    if (as.op == op) {
      return as.func;
    }
  }
  return add;
}

int main() {
  int num1 = getInt();
  int num2 = getInt();
  char op = getOp();

  std::cout << num1 << op << num2 << " = " << getArithmeticFunction(op)(num1, num2) << "\n";
}
