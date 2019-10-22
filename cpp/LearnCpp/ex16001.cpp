#include <iostream>
#include <vector>
#include <deque>
#include <list>
#include <string>
#include <map>
#include <algorithm>

int main() {
  {
    std::vector<int> vect;
    for (int count{0}; count < 6; ++count) {
      vect.push_back( 10 - count );
    }
    for (const auto& v: vect)
      std::cout << v << " ";
    std::cout << "\n";
    std::vector<int>::const_iterator it = vect.cbegin();
    while (it != vect.end()) {
      std::cout << *it << "(it) ";
      ++it;
    }
    std::cout << "\n";

    std::list<int> li;
    for (int count{0}; count < 6; ++count) {
      li.push_back(count);
    }
    std::list<int>::const_iterator lit = li.cbegin();
    int sum = 0;
    while (lit != li.end()) {
      sum += *lit;
      ++lit;
    }
    std::cout << "sum of list is: " << sum << "\n";

    std::deque<int> deq;
    for (int count {0}; count < 6; ++count) {
      deq.push_back(count);
      deq.push_front(10-count);
    }
    for (const auto& v: deq)
      std::cout << v << " ";
    std::cout << "\n";

    std::map<std::string, int> map;
    map.insert(std::make_pair("apple", 4));
    map.insert(std::make_pair("orange", 2));
    map.insert(std::make_pair("peach", 5));
    map.insert(std::make_pair("banana", 1));
    for (const auto& m : map) {
      std::cout << m.first << "=" << m.second << " ";
    }
    std::cout << "\n";
  }

  {
    std::cout << "======================= show of algorithm\n";
    std::list<int> li {7, 2, 3, 4, 5, 1, 9};
    auto it = find(li.begin(), li.end(), 3);
    li.insert(it, 8);
    std::cout << "min value: " << *(min_element(li.begin(), li.end())) << "\n";
    std::cout << "max value: " << *(max_element(li.begin(), li.end())) << "\n";
  }
  {
    std::vector<double> vec {1.2, 3.4, 0.8, 7.4, -0.3, -12};

    std::sort(vec.begin(), vec.end());
    for (auto& v : vec) {
      std::cout << v << " ";
    }
    std::cout << "\n";
    std::reverse(vec.begin(), vec.end());
    for (auto& v : vec) {
      std::cout << v << " ";
    }
    std::cout << "\n";
  }

  return 0;
}
