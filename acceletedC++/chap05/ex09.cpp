#include <vector>
#include <iostream>
#include <algorithm>

using namespace std;
bool compare(const char& c1, const char& c2) {
  return c1 > c2;
}

int main() {
  string input="SomeString";

  vector<char> input_char(input.begin(), input.end());
  sort(input_char.begin(), input_char.end(), compare);

  for (vector<char>::const_iterator it=input_char.begin(); it != input_char.end(); ++it)
    cout << *it;

  cout << endl;
  return 0;
}
