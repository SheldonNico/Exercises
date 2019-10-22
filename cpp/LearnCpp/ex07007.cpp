#include <iostream>
#include <cassert>
#include <array>

void printString(const char* cstring) {
  if (cstring)
    std::cout << cstring;
}

int getArrayValue(const std::array<int, 10>& array, int index) {
  assert(index >= 0 && index <= 9 && "index must be inside 0-9");

  return array[index];
}

int main(int argc, char* argv[]) {
  char cstring[10];

  printString(cstring);
  std::cout << "Finished!\n";

  std::array<int, 10> array { {1, 2, 3,} };
  std::cout << getArrayValue(array, 2);

  for (int count=0; count < argc; ++count) {
    std::cout << count << ": " << argv[count] << "\n";
  }

  return 0;
}
