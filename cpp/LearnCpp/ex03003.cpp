#include "ex03003_h.h"
#include <iostream>

double fall_length(const int& sec) {
  return gravity::gravity_const * sec * sec / 2;
}

int main () {
  std::cout << "Enter the height of the tower in meters: ";
  int height {0};
  std::cin >> height;
  int sec = 0;
  double currentPos = 0;

  while (currentPos < height) {
    std::cout << "At " << sec << " seconds, the ball is at height: " <<
      height - currentPos << " meters \n";
    sec += 1;
    currentPos = fall_length(sec);
  }
  std::cout << "At " << sec << " seconds, the ball is on the ground.\n";

  return 0;
}
