#include <string>
#include <vector>
#include <iostream>
#include <map>

using namespace std;

map< int, vector<string> > find_freq(vector<string>& words) {
  map< int, vector<string> > ret;
  map< string, int > freqs;

  for (vector<string>::const_iterator it = words.begin(); it != words.end(); ++it) {
    freqs[*it] += 1;
  }

  for (map< string, int >::const_iterator it = freqs.begin(); it != freqs.end(); ++it) {
    ret[it->second].push_back(it->first);
  }

  return ret;
}

int main() {
  vector<string> words;
  string word;

  while (cin >> word) {
    words.push_back(word);
  }

  map< int, vector<string> > wordFreq = find_freq(words);
  for (map< int, vector<string> >::const_iterator it=wordFreq.begin();
       it != wordFreq.end();
       ++it) {
    cout << it->first << ": ";
    vector<string>::const_iterator iter = it->second.begin();
    cout << *iter;
    ++iter;

    while (iter != it->second.end()) {
      cout << ", " << *iter;
      ++iter;
    }
    cout << endl;
  }

  return 0;
}
