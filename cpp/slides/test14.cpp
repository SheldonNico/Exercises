#include <iostream>
#include <map>
#include <vector>

using namespace std;

int main() {
  vector<double> x = {1, 2, 3};

  for (auto it = x.begin(); it != x.end(); ++it) {
    cout << *it << endl;
  }

  map<int, string> m = {{1, "hello"}, {2.0, "world"}};
  auto m_it = m.find(1);
  if (m_it != m.end()) {
    cout << m_it->first << ": " << m_it->second << endl;
  }
  if (m.find(3) == m.end()) {
    cout << "3 not found inside map" << endl;
  }

}
