#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {
  vector<string> words;
  vector<int> counts;

  for (string word; cin >> word; ) {
    bool found = false;
    vector<string>::size_type size = words.size();
    for (int i = 0; i < size; i++) {
      if (words[i] == word) {
        counts[i]++;
        found = true;
        break;
      }
    }

    if (!found) {
      words.push_back(word);
      counts.push_back(1);
    }
  }

  vector<string>::size_type size = words.size();
  for (int i = 0; i < size; i++) {
    cout << "counts of " << words[i] << ":" << counts [i] << endl;
  }

  return 0;
}
