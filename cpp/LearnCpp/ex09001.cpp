#include <iostream>

class Cents {
  private:
    int m_cents;

  public:
    Cents(int cents) : m_cents {cents} {};
    friend Cents operator+(const Cents& c1, const Cents& c2);
    friend Cents operator+(const Cents& c1, const int& c2);
    friend Cents operator+(const int& c2, const Cents& c1);
    friend Cents operator-(const Cents& c1, const Cents& c2);

    int getCents() const { return m_cents; } ;
};

Cents operator+(const Cents& c1, const Cents& c2) {
  return Cents { c1.m_cents + c2.m_cents };
}
Cents operator+(const Cents& c1, const int& c2) {
  return { c1.m_cents + c2 };
}
Cents operator+(const int& c2, const Cents& c1);
Cents operator-(const Cents& c1, const Cents& c2) {
  return Cents { c1.m_cents - c2.m_cents };
}

int main() {
  Cents cents1(6);
  Cents cents2(8);
  Cents centsSum = cents1 + cents2;
  std::cout << "I have " << centsSum.getCents() << " cents." << std::endl;
  std::cout << "I have " << (Cents {5} + 2).getCents() << " cents." << std::endl;

  return 0;
}
