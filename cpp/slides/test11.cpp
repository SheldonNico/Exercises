#include <fstream>
#include <vector>

int main() {
  std::string file_name = "/tmp/image.dat";
  std::ofstream file(file_name, std::ios_base::out | std::ios_base::binary);
  if (!file) { return EXIT_FAILURE; }

  int r = 2; int c = 3;
  std::vector<float> vec(r*c, 23);
  file.write(reinterpret_cast<char*>(&r), sizeof(r));
  file.write(reinterpret_cast<char*>(&c), sizeof(c));
  // file.write(reinterpret_cast<char*>(&vec), sizeof(vec)); // wrong version, address should be the start address
  file.write(reinterpret_cast<char*>(&vec.front()), vec.size()*sizeof(vec.front()));

  return 0;
}
