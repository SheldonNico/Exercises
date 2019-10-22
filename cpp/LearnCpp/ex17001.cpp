#include <string>
#include <iostream>
#include <sstream>

int main() {
  std::cout << std::string("my string") << "\n";
  std::cout << std::string("my string", 4) << "\n";
  std::cout << std::string("my string", 4, 5) << "\n";
  std::cout << std::string(5, 'Q') << "\n";

  std::ostringstream oStream;
  oStream << 5;
  std::cout << oStream.str() << "\n";

  std::cout << "string max length: " << (std::string {"example"}).max_size() << "\n";
}
