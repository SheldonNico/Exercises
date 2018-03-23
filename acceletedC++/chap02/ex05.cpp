#include <iostream>
#include <string>

using std::cin;
using std::cout;
using std::endl;

int main() {
  const char sym = '*';
  const std::string::size_type size = 9;

  cout << "Square:" << endl;
  for (int r=0; r != size; ++r) {
    std::string::size_type c = 0;
    while (c != size) {
      if (r == 0 || c == 0 || r == size-1 || c == size-1) {
        cout << sym;
      } else
        cout << ' ';
      ++ c;
    }
    cout << endl;
  }

  cout << "Triangle:" << endl;
  int start, end;
  for (int r=0; r != size; ++r) {
    start = size - r;
    end = 2*size - start;
    for (int c=0; c != 2*size; ++c) {
      if (c >= start && c <= end)
        cout << sym;
      else
        cout << ' ';
    }
    cout << endl;
  }

  const std::string::size_type width=2*size;
  const std::string::size_type height=size;

  cout << "Rectangle: " << endl;
  for (int r=0; r != height; ++r) {
    for (int c=0; c!= width; ++c) {
      if (r == 0 || c == 0 || r == height-1 || c ==width-1)
        cout << sym;
      else
        cout << ' ';
    }
    cout << endl;
  }

  return 0;
}
