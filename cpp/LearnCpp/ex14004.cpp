#include <iostream>

class Fraction {
  private:
    int m_num, m_den;

  public:
    Fraction(int num, int den) : m_num{num}, m_den{den} {
      if (den == 0)
        throw std::runtime_error("denocimal number should not be 0");
    }
};

int main() {

  int num, den;
  std::cout << "Enter the numerator: ";
  std::cin >> num;
  std::cout << "Enter the denominator: ";
  std::cin >> den;
  try {
    Fraction f {num, den};
  } catch (const std::runtime_error& err) {
    std::cout << "Your fraction has an invalid denominator.\n";
  }

  return 0;
}
