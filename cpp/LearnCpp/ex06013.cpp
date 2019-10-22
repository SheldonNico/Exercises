#include <iostream>

void printString(const char* c) {
  while (*c != '\0') {
    std::cout << *c;
    ++c;
  }

}


int main() {
  char string[] {"Hello world!"};
  string[5] = '\0';
  printString(string);
  std::cout << "\n";

  return 0;
}
