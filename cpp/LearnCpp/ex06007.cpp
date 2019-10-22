#include <iostream>
#include <memory>
#include <string>

int main() {
  std::string names[] { "Alex", "Betty", "Caroline", "Dave", "Emily", "Fred", "Greg", "Holly", };
  std::cout << "Enter a name: ";

  std::string name {};
  std::cin >> name;

  bool found {false};
  for (auto& n : names) {
    if (name == n) {
      found = true;
      break;
    }
  }
  std::cout << name << (found ? " was found\n" : " was not found\n");
  return 0;
}
