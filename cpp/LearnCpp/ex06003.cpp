#include <utility>
#include <iostream>
#include <cstring>
#include <iterator>

int main() {
  const int numRows = 10;
  const int numCols = 10;
  int product[numRows][numCols] = { 0 };


  for (int row=0; row < numRows; ++row) {
    for (int col=0; col < numCols; ++col) {
      product[row][col] = row *  col;
    }
  }

  for (int row=0; row < numRows; ++row) {
    for (int col=0; col < numCols; ++col) {
      std::cout << product[row][col] << "\t";
    }
    std::cout << "\n";
  }

  // C-style string
  char myString[] = "string";
  std::cout << std::size(myString) << " last char: " << static_cast<int>(myString[6]) << "\n";

  char source[] = "Copy this";
  char dest[5];
  strcpy(dest, source);
  std::cout << dest << "\n";
  std::cout << "dest now become: " << dest << '\n';
  std::cout << source << " has " << strlen(dest) << " letters\n";
  std::cout << source << " has " << std::size(dest) << " elements\n";

  char name[255] = "";
  std::cout << "Enter your name: ";
  std::cin.getline(name, 255);
  std::cout << "You entered: " << name << "\n";
  return 0;
}
