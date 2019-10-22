#include <iostream>

class Cents {
  private:
    int m_cents;

  public :
    Cents(int cents = 0) : m_cents {cents} { }

    operator int() { return m_cents; }
};

class Dollars {
  private:
    int m_dollars;

  public:
    Dollars(int dollars=0) : m_dollars {dollars} {}

    operator Cents() { return {m_dollars*100}; }
};

int main() {
  Dollars d {12};

  std::cout << static_cast<int>(static_cast<Cents>(d));
  return 0;
}
