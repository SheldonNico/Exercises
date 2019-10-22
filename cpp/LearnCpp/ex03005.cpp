#include <iostream>
#include <cmath>

using namespace std;

void printBin(int num) {
  cout << num << ": ";

  for (int i=7; i >= 0; i--) {
    if (i == 3)
      cout << " ";

    int rem = pow(2, i);
    if (num >= rem) {
      cout << "1";
      num = num % rem;
    } else {
      cout << "0";
    }
  }
  cout << "\n";

}

int main() {
  for (int i=5; i <= 12; i++) {
    printBin(i);
  }

  const unsigned char option0 = 0b0000'00001;
  const unsigned char option1 = 0b0000'00011;

  return 0;
}
