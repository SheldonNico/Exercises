#include <string>
#include <map>
#include <vector>
#include <iostream>
#include <algorithm>
#include <cctype>
#include <stdexcept>
#include <cstdlib>

using namespace std;

bool is_space(char c) {
  return isspace(c);
}

bool not_space(char c) {
  return !is_space(c);
}

vector<string> split(const string& line) {
  vector<string> ret;

  string::const_iterator i = line.begin();
  string::const_iterator j;
  while (i != line.end()) {
    i = find_if(i, line.end(), not_space);
    j = find_if(i, line.end(), is_space);
    if (i != line.end())
      ret.push_back(string(i, j));

    i = j;
  }

  return ret;
}

map< string, vector<int> > xref(istream& in, vector<string> find_words(const string&) = split) {
  string line;
  int line_number = 0;
  map<string, vector<int>> ret;

  while (getline(in, line)) {
    ++line_number;

    vector<string> words = find_words(line);
    for (vector<string>::const_iterator it=words.begin(); it != words.end(); ++it) {
      vector<int>& counts = ret[*it];
      if (find(counts.begin(), counts.end(), line_number) == counts.end())
        counts.push_back(line_number);
    }
  }
  return ret;
}

int main() {
  map<string, vector<int>> out = xref(cin);

  for (map<string, vector<int>>::const_iterator it=out.begin();
       it != out.end();
       ++it) {
    cout << it->first << " occurs @: ";

    cout << it->second[0];
    for (vector<int>::const_iterator iter=it->second.begin()+1; iter!=it->second.end(); ++iter)
      cout << " " << *iter;
    cout << endl;
  }

  return 0;
}
