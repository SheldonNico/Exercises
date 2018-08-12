#include <iostream>
#include <list>
#include <string>

using namespace std;

bool is_palindrome(const string& p) {
  string rev(p.rbegin(), p.rend());
  return rev == p;
}

int main() {
  string line;
  list<string> dict, pdict;
  while (getline(cin, line))
    dict.push_back(line);

  string::size_type maxlen = 0;
  string maxp;

  for (list<string>::const_iterator it=dict.begin(); it != dict.end(); ++it) {
    if (is_palindrome(*it)) {
      pdict.push_back(*it);
      if (it->size() > maxlen) {
        maxp = *it;
        maxlen = it->size();
      }

      cout << *it << endl;
    }
  }

  cout << endl << "maxlen palindrome: " << maxp << " @length:" << maxlen << endl;

  return 0;
}
