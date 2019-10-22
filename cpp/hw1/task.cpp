#include <iostream>
#include <cstdlib>


int main() {
  int target = std::rand() / ((RAND_MAX + 1u) / 99);
  int guess;
  std::cout << target <<  std::endl;

  while (true) {
    std::cout << "Please input your guess (between 0 and 99) ";
    std::cin >> guess;

    if (guess == target) {
      std::cout << "You won" << std::endl;
      break;
    } else if (guess < target) {
      std::cout << "Your guess [" << guess << "] is smaller." << std::endl;
    } else {
      std::cout << "Your guess [" << guess << "] is larger." << std::endl;
    }
 }

  return 0;
}
