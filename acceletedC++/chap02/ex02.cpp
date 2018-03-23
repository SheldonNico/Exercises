#include <iostream>
#include <string>

using std::cin;
using std::cout;
using std::string;
using std::endl;

int main() {
  cout << "Please enter your first name: ";

  string name;
  cin >> name;

  const string greeting = "Hello, " + name + "!";
  const int pad_h = 1;
  const int pad_w = 1;
  const int rows = pad_h*2 + 3;
  const string::size_type cols = greeting.size() + pad_w*2 + 2;

  // int r = 0;
  // while (r != rows) {
  //   std::cout << std::endl;
  //   ++r;
  // }
  cout << endl;
  for (int r=0; r != rows; ++r) {
    string::size_type c = 0;
    while (c != cols) {
      if (r == pad_h+1 && c == pad_w+1) {
        cout << greeting;
        c += greeting.size();
      } else {
        if (r == 0 || r == rows-1 || c == 0 || c == cols-1)
          cout << "*";
        else
          cout << " ";
        ++c;
      }
    }
    cout << endl;
  }

  return 0;
}
