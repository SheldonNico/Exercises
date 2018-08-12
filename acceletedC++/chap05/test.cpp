#include <vector>
#include <iostream>
#include <tuple>
#include <string>
#include <list>

using namespace std;

int main() {
  list< tuple<int, string> > time;
  time.push_back(make_tuple(1, "ab"));
  time.push_back(make_tuple(2, "xy"));

  for (list<tuple<int, string>>::const_iterator it=time.begin(); it != time.end(); ++it) {
    cout << get<0>(*it) << endl;
  }
  return 0;
}
