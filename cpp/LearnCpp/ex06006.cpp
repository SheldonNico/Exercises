#include <iostream>
#include <algorithm>
#include <string>

int main() {
  std::cout << "how many names will you enter: ";
  int numOfName = 0;
  std::cin >> numOfName;

  auto names = new std::string[numOfName] {};

  for (int i = 0; i < numOfName; ++i) {
    std::cout << "Enter name #" << i+1 << ": ";
    std::cin >> names[i];
  }

  std::sort(names, names+numOfName);

  for (int i = 0; i < numOfName; ++i) {
    std::cout << names[i] << " ";
  }

  std::cout << "\n";
  delete[] names;

  return 0;
}
