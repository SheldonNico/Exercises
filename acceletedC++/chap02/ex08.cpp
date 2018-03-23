#include <iostream>
#include <string>

using std::cin;
using std::cout;
using std::endl;

int main() {
  int product = 1;
  for (int i=1; i != 10; ++i)
    product *= i;
  cout << "Product is: " << product << endl;

  return 0;
}
