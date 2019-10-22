#include <iostream>
#include <cassert>

class Fraction {
  private:
    int m_numerator, m_denominator;

  public:
    Fraction(int numerator=0, int denominator=1) : m_numerator{numerator}, m_denominator{denominator} {
      assert( m_denominator != 0 );
    }

    Fraction(const Fraction& copy) : m_numerator{copy.m_numerator}, m_denominator{copy.m_denominator} {
      std::cout << "copy constructor is called!" << m_numerator << " " << m_denominator << "\n";
    }

    friend std::ostream& operator<< (std::ostream& os, const Fraction& f) {
      os << f.m_numerator << "/" << f.m_denominator;
      return os;
    }
    friend Fraction makeNegative(Fraction f);

    Fraction& operator= (const Fraction& frac) {
      if (this != &frac) {
        this->m_numerator = frac.m_numerator;
        this->m_denominator = frac.m_denominator;
      }
      return *this;
    }
};

Fraction makeNegative(Fraction f) {
  f.m_numerator = - f.m_numerator;
  return f;
}

class MyString {
  private:
    std::string m_string;

  public:
    //MyString(char) = delete;

    explicit MyString(int x) {
    //MyString(int x) {
      m_string.resize(x);
    }

    MyString(const char* string) {
      m_string = string;
    }

    friend std::ostream& operator<<(std::ostream& os, const MyString& s) {
      os << s.m_string;
      return os;
    }

    MyString& operator= (const MyString& str) {
      if (this == &str)
        return *this;
      m_string = str.m_string;

      return *this;
    }
};


int main() {

  Fraction fiveThirds {5, 3};
  std::cout << makeNegative(fiveThirds) << "\n";
  std::cout << makeNegative(5) << "\n";

  // MyString mine = 'x';
  MyString mine = static_cast<MyString>('x');
  std::cout << mine;

  Fraction five = {1, 3}; // copy constructor
  Fraction f;
  f = {1, 3}; // copy assignment

  return 0;
}
