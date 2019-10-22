#include <fstream>
#include <vector>
#include <iostream>

int main() {
  std::string file_name = "/tmp/image.dat";
  std::ifstream file(file_name, std::ios_base::in | std::ios_base::binary);
  if (!file) { return EXIT_FAILURE; }

  int r, c;
  file.read(reinterpret_cast<char*>(&r), sizeof(r));
  file.read(reinterpret_cast<char*>(&c), sizeof(c));
  std::vector<float> vec(r*c);
  file.read(reinterpret_cast<char*>(&vec.front()), vec.size()*sizeof(vec.front()));

  for (auto d : vec) {
    std::cout << d << " ";
  }
  std::cout << std::endl;

  return 0;
}
