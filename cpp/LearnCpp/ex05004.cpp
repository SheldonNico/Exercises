#include <iostream>
#include <ctime>
#include <random>

namespace RandomSeed {
  std::mt19937 mersenne(static_cast<std::mt19937::result_type>(std::time(nullptr)));
}

int getRandomNumber(int min, int max) {
  std::uniform_int_distribution<> die(min, max);
  return die(RandomSeed::mersenne);

}

int main() {
  for (int count = 1; count <=48; ++count) {
    std::cout << getRandomNumber(1, 6) << "\t";
    if (count % 6 == 0) std::cout << "\n";
  }

  return 0;
}
