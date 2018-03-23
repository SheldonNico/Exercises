#include <string>
#include <iostream>

using namespace std;

int main() {
  string longest, shortest, word;
  string::size_type lenLong, lenShort;

  cin >> word;
  longest = shortest = word;
  lenLong = lenShort = word.size();

  for (string word; cin >> word; ) {
    string::size_type len = word.size();
    if (len > lenLong) {
      longest = word;
      lenLong = len;
    }
    if (len < lenShort) {
      shortest = word;
      lenShort = len;
    }
  }

  cout << "longest string is: " << longest << "@" << lenLong << endl;
  cout << "shortest string is: " << shortest << "@" << lenShort << endl;

  return 0;
}
