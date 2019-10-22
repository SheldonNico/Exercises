#include <iostream>
#include <iomanip>
#include <typeinfo>
#include <string>

int main() {
  std::cout << "Enter your full name: ";
  std::string name;
  std::getline(std::cin, name);

  std::cout << "Enter your age: ";
  int age;
  std::cin >> age;

  float avg_age = static_cast<float>(age) / name.length();
  std::cout << "You've lived "<< avg_age << " years for each letter in your name.\n";
  return 0;
}
