#include <iostream>
#include <string>
#include <vector>
#include <typeinfo>

template<typename T> T read() {
  std::cout << "Please enter a " << typeid(T).name() << ":";
  T out {};
  std::cin >> out;

  return out;
}

int main () {
  std::cout << "Please enter a double: ";
  float num1;
  std::cin >> num1;

  std::cout << "Please enter a double: ";
  float num2;
  std::cin >> num2;

  std::cout << "Please one of following operator: +, -, *, or /: ";
  char op;
  std::cin >> op;

  if ( op == '+')
    std::cout << num1 << " " << op << " " << num2 << " = " << num1+num2;
  else if ( op == '-')
    std::cout << num1 << " " << op << " " << num2 << " = " << num1-num2;
  else if ( op == '*')
    std::cout << num1 << " " << op << " " << num2 << " = " << num1*num2;
  else if ( op == '/')
    std::cout << num1 << " " << op << " " << num2 << " = " << num1/num2;
  std::cout << "\n";

  return 0;
}
