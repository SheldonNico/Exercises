#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using std::cin;
using std::cout;
using std::endl;
using std::vector;
using std::istream;
using std::ostream;
using std::string;
using std::find;

istream& read_words(istream& in, vector<string>& store, vector<int>& freqs) {
  store.clear(); freqs.clear();
  string name;
  while (in >> name) {
    auto idx = find(store.begin(), store.end(), name);
    if (idx != store.end()) {
      freqs[idx-store.begin()] += 1;
    } else {
      store.push_back(name);
      freqs.push_back(1);
    }
  }
  in.clear();

  return in;
}

int main() {
  vector<string> names;
  vector<int> freqs;
  read_words(cin, names, freqs);
  cout << "the number of word:" << names.size() << endl;
  for (int i=0; i < names.size(); ++i) {
    cout << names[i] << ": " << freqs[i] << endl;
  }

  return 0;
}
