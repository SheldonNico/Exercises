#include <map>
#include <unordered_map>
#include <iostream>

int main() {
  auto map = std::unordered_map<int, char>{
    {0, 'a'}, {1, 'b'}, {4, 'z'}};

  for (auto it = map.begin(); it != map.end(); ++it)
    std::cout << it->first << ", " << it->second << std::endl;

  std::cout << "Use auto:" << std::endl;
  for (const auto& m : map)
    std::cout << m.first << ", " << m.second << std::endl;

  return 0;
}
