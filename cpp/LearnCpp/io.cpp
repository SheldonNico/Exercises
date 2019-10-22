#include "io.h"

int readNumber() {
  int out{0};
  std::cout << "Please enter you integer: ";

  std::cin >> out;
  return out;
}

void writeAnswer(int ans) {
  std::cout << "answer is " << ans << "\n";
}
