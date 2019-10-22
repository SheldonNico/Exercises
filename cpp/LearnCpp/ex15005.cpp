#include <iostream>
#include <memory>

class Fraction {
  private:
    int m_numerator = 0, m_denominator = 1;

  public:
    Fraction(const int& numerator = 0, const int& denominator = 1) : m_numerator{numerator}, m_denominator{denominator} {}

    friend std::ostream& operator<<(std::ostream& os, const Fraction& frac) {
      os << frac.m_numerator << "/" << frac.m_denominator;
      return os;
    }
};

void printFraction(const Fraction& ptr) {
  std::cout << ptr;
}

int main() {
  auto ptr = std::make_unique<Fraction>(3, 5);
  printFraction(*ptr);

  return 0;
}
