#include <bitset>
#include <iostream>

const int option0 = 0;
const int option1 = 1;
const int option2 = 2;
const int option3 = 3;
const int option4 = 4;
const int option5 = 5;
const int option6 = 6;
const int option7 = 7;

int main() {
  std::bitset<8> bits(0b0000'0010);
  bits.set(option4);
  bits.flip(option5);
  bits.reset(option5);

  std::cout << "Bit 4 has value: " << bits.test(option4) << "\n";
  std::cout << "Bit 5 has value: " << bits.test(option5) << "\n";
  std::cout << "all the bits: " << bits << "\n";
  return 0;
}
