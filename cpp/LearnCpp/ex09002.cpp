#include <iostream>
#include <cassert>
#include <numeric>

class Fraction {
  private:
    int m_numerator, m_denominator;

  public:
    Fraction(const int& n=0, const int& d=1) : m_numerator{n}, m_denominator{d} {
      assert(m_denominator != 0 && "denominator must not be zero.");
    }

    void print() const {
      std::cout << m_numerator << "/" << m_denominator << "\n";
    }

    friend Fraction operator*(const Fraction& f1, const Fraction& f2);
    friend Fraction operator*(const Fraction& f1, const int& f2);
    friend Fraction operator*(const int& f1, const Fraction& f2);
    friend std::ostream& operator<<(std::ostream& os, const Fraction& f);
    friend std::istream& operator>>(std::istream& is, Fraction& f);
};

std::ostream& operator<<(std::ostream& os, const Fraction& f) {
  os << f.m_numerator << "/" << f.m_denominator;
  return os;
}

std::istream& operator>>(std::istream& is, Fraction& f) {
  is >> f.m_numerator >> f.m_denominator;
  return is;
}

int gcd(const int& a, const int& b) {
  return (b == 0) ? (a > 0 ? a : -a) : (gcd(b, a % b));
}

Fraction operator*(const Fraction& f1, const Fraction& f2) {
  int n = f1.m_numerator * f2.m_numerator;
  int d = f1.m_denominator * f2.m_denominator;
  int c = gcd(n, d);
  return {n / c, d / c};
}
Fraction operator*(const int& f1, const Fraction& f2) {
  return Fraction {f1, 1} * f2;
}
Fraction operator*(const Fraction& f1, const int& f2) {
  return f2 * f1;
}


int main() {
  Fraction(2, 5).print();
  Fraction(3, 8).print();
  (Fraction {2, 5} * Fraction { 3, 8 }).print();
  (2 * Fraction { 2, 5 }).print();
  (Fraction {3, 8} * 2).print();
  (Fraction {1, 2} * Fraction { 2, 3 } * Fraction { 3, 4 }).print();

  Fraction f1;
  std::cout << "Enter fraction 1: ";
  std::cin >> f1;

  Fraction f2;
  std::cout << "Enter fraction 2: ";
  std::cin >> f2;

  std::cout << f1 << " * " << f2 << " is " << f1 * f2 << '\n'; // note: The result of f1 * f2 is an r-value

  return 0;
}
