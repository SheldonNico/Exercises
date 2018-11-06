// using std=c++11 flag
#include <map>
#include <vector>
#include <algorithm>
#include <string>
#include <stdexcept>
#include <iostream>

using namespace std;
map<string, pair<int, int>> levels = {
  {"A", {90, 100}},
  {"B", {80, 90}},
  {"C", {60, 70}},
  {"D", {0, 60}}
};

string find_grades(double score) {
  for (map<string, pair<int, int>>::const_iterator it = levels.begin();
       it != levels.end();
       ++it) {
    double up, down;
    down = it->second.first;
    up = it->second.second;

    if (down <= score && score < up)
      return it->first;
  }

  throw logic_error("score must between 0 and 100(not included)");
}

int main() {
  cout << find_grades(100) << endl;

  return 0;
}
