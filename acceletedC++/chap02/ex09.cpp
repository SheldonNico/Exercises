#include <iostream>
#include <string>

using std::cin;
using std::cout;
using std::endl;

int main() {
  int a, b;
  cout << "Please enter your first number: "; cin >> a;
  cout << "Please enter your second number: "; cin >> b;
  if (a > b)
    cout << a << ">" << b << endl;
  else {
    if (a < b)
      cout << a << "<" << b << endl;
    else
      cout << a << "=" << b << endl;
  }

  return 0;
}
