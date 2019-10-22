#include <iostream>
#include <cstdlib>
#include <ctime>

unsigned int PRNG(unsigned int& seed) {
  seed = 8253729 * seed + 2396403;

  return seed % 32768;
}

int getRandomNumber(int min, int max) {
  static const double fraction = 1.0 / (RAND_MAX + 1.0);
  return min + static_cast<int>((max-min+1)*(std::rand() * fraction));
}

auto sum(const int& num) -> int {
  int result = 0;
  for (int i = 0; i < num+1; i++) {
    result += i;
  }
  return result;
}

int main() {
  std::srand(static_cast<unsigned int>(std::time(nullptr)));

  for (char c = 'a'; c <= 'z'; ++c) {
    std::cout << c << " => " << static_cast<int>(c) << "\n";
  }

  std::cout << "================\n";

  for (int i = 0; i < 5; i ++) {
    for (int j = 5-i; j > 0; j--) {
      std::cout << j << " ";
    }
    std::cout << "\n";
  }

  std::cout << "================\n";

  for (int i = 0; i < 5; i++) {
    for (int j = 5; j > 0; j--) {
      if (j <= i+1)
        std::cout << j << " ";
      else
        std::cout << ' ' << " ";
    }
    std::cout << "\n";
  }

  std::cout << "================\n";
  for (int i = 0; i < 21; i++) {
    std::cout << i << " ";
  }
  std::cout << "\n";

  std::cout << "================\n";
  unsigned int seed = 5323;
  for (int i = 1; i <= 100; ++i) {
    std::cout << PRNG(seed) << " ";

    if (i % 10 == 0)
      std::cout << "\n";
  }

  std::cout << "================\n";
  for (int count = 1; count <= 100; ++count) {
    std::cout << std::rand() << " ";

    if (count % 10 == 0)
      std::cout << "\n";
  }

  std::cout << "================\n";
  for (int count = 1; count <= 100; ++count) {
    std::cout << getRandomNumber(1, 6) << " ";

    if (count % 10 == 0)
      std::cout << "\n";
  }
  return 0;
}
