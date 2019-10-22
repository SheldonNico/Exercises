#include <array>
#include <iostream>
#include <vector>



void printLength(const std::array<double, 5>& array) {
  for (const auto& a : array)
    std::cout << a << " ";
  std::cout << "\n";
  std::cout << "length: " << array.size() << " :" << array.at(0) << "\n";
}

int main() {
  std::array<int, 3> array { 1, 9, 7 };
  std::cout << array[0] << "\n";
  std::cout << array.at(2) << "\n";
  std::cout << array.size() << "\n";


  std::array<double, 5> array2 { 1, 9, 7 };
  printLength(array2);

  using index_t = std::array<int, 5>::size_type;
  for (index_t i {0}; i < array.size(); ++i)
    std::cout << "working on " << i << "\n";

  std::vector<int> vec { 9, 7, 5, 3, 1 };
  std::cout << vec[0] << vec.at(0) << vec.size() << "\n";
  vec = { 0, 1, 2, };
  std::cout << vec[0] << vec.at(0) << vec.size() << "\n";

  std::vector<bool> barray { true, false, false, true, true };
  for (auto const& e : barray)
    std::cout << e << " ";
  std::cout << "\n";

  return 0;
}
