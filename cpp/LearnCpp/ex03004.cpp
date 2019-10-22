#include <iostream>
#include <cmath>
#include <string>

int main () {
  std::cout << "Enter an integer: ";
  int x { 0 };
  std::cin >> x;

  std::cout << "Enter another integer: ";
  int y { 0 };
  std::cin >> y;

  if (x == y)
    std::cout << x << " equals " << y << "\n";
  if (x != y)
    std::cout << x << " does not equal " << y << "\n";
  if (x > y)
    std::cout << x << " is greater than " << y << "\n";


  return 0;
}
