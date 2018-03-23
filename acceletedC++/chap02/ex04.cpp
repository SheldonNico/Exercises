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
  const int pad = 1;
  const int rows = pad*2 + 3;
  const string::size_type cols = greeting.size() + pad*2 + 2;
  const string blankLine(cols-2, ' ');

  // int r = 0;
  // while (r != rows) {
  //   std::cout << std::endl;
  //   ++r;
  // }
  cout << endl;
  for (int r=0; r != rows; ++r) {
    string::size_type c = 0;
    while (c != cols) {
      if (r == pad+1 && c == pad+1) {
        cout << greeting;
        c += greeting.size();
      } else {
        if (r == 0 || r == rows-1 || c == 0 || c == cols-1) {
          cout << "*";
          ++c;
        }
        else {
          if (r == pad+1) {
            cout << " ";
            ++c;
          } else {
            cout << blankLine;
            c += blankLine.size();
          }
        }
      }
    }
    cout << endl;
  }

  return 0;
}
