#include <iostream>
#include <cassert>
#include <stdlib.h>
#include <cmath>

class FixedPoint2 {
  private:
    int m_dec;
    int m_frac;
    int m_sign = 1;

  public:
    FixedPoint2(int dec, int frac=0) {
      if (frac * dec < 0)
        m_sign = -1;
      else if (frac * dec == 0) {
        if (frac < 0 || frac < 0)
          m_sign = -1;
        else
          m_sign = 0;
      }

      m_dec = dec > 0 ? dec : -dec;
      m_frac = frac > 0 ? frac : -frac;

      assert ( m_frac <= 99 && "this is only for two decimal");
    }

    operator double() const {
      return m_sign * (m_dec + m_frac / 100.0);
    }

    FixedPoint2(double d) {
      if (d < 0) {
        d = -d;
        m_sign = -1;
      } else if (d == 0) {
        m_sign = 0;
      }

      m_dec = static_cast<int>(d);
      d -= m_dec;
      m_frac = std::round(100 * d);
    }

    friend bool operator== (const FixedPoint2& fp1, const FixedPoint2& fp2);
    friend FixedPoint2 operator+ (const FixedPoint2& fp1, const FixedPoint2& fp2);
    friend std::ostream& operator<< (std::ostream& os, const FixedPoint2& fp);
    friend std::istream& operator>> (std::istream& is, FixedPoint2& fp);
    FixedPoint2 operator- () const {
      return {-m_sign * m_dec, m_frac};
    }
};

std::ostream& operator<< (std::ostream& os, const FixedPoint2& fp) {
  if (fp.m_sign < 0)
    os << "-";

  os << fp.m_dec << ".";
  if (fp.m_frac < 10)
    os << '0' << fp.m_frac;
  else
    os << fp.m_frac;

  return os;
}
std::istream& operator>> (std::istream& is, FixedPoint2& fp) {
  double tmp;
  is >> tmp;
  fp =  static_cast<FixedPoint2>(tmp);
  return is;
}
bool operator== (const FixedPoint2& fp1, const FixedPoint2& fp2) {
  return (fp1.m_sign == fp2.m_sign) && (fp1.m_dec == fp2.m_dec) && (fp1.m_frac == fp2.m_frac);
}
FixedPoint2 operator+ (const FixedPoint2& fp1, const FixedPoint2& fp2) {
  return static_cast<double>(fp1) + static_cast<double>(fp2);
}


void testAddition()
{
	// h/t to reader Sharjeel Safdar for this function
	std::cout << std::boolalpha;
	std::cout << (FixedPoint2(0.75) + FixedPoint2(1.23) == FixedPoint2(1.98)) << '\n'; // both positive, no decimal overflow
	std::cout << (FixedPoint2(0.75) + FixedPoint2(1.50) == FixedPoint2(2.25)) << '\n'; // both positive, with decimal overflow
	std::cout << (FixedPoint2(-0.75) + FixedPoint2(-1.23) == FixedPoint2(-1.98)) << '\n'; // both negative, no decimal overflow
	std::cout << (FixedPoint2(-0.75) + FixedPoint2(-1.50) == FixedPoint2(-2.25)) << '\n'; // both negative, with decimal overflow
	std::cout << (FixedPoint2(0.75) + FixedPoint2(-1.23) == FixedPoint2(-0.48)) << '\n'; // second negative, no decimal overflow
	std::cout << (FixedPoint2(0.75) + FixedPoint2(-1.50) == FixedPoint2(-0.75)) << '\n'; // second negative, possible decimal overflow
	std::cout << (FixedPoint2(-0.75) + FixedPoint2(1.23) == FixedPoint2(0.48)) << '\n'; // first negative, no decimal overflow
	std::cout << (FixedPoint2(-0.75) + FixedPoint2(1.50) == FixedPoint2(0.75)) << '\n'; // first negative, possible decimal overflow
}

int main() {

  //FixedPoint2 a(34, 56);
  //std::cout << a << '\n';

  //FixedPoint2 b(-2, 8);
  //std::cout << b << '\n';

  //FixedPoint2 c(2, -8);
  //std::cout << c << '\n';

  //FixedPoint2 d(-2, -8);
  //std::cout << d << '\n';

  //FixedPoint2 e(0, -5);
  //std::cout << e << '\n';

  //std::cout << static_cast<double>(e) << '\n';

  //FixedPoint2 a(0.01);
  //std::cout << a << '\n';

  //FixedPoint2 b(-0.01);
  //std::cout << b << '\n';

  //FixedPoint2 c(5.01); // stored as 5.0099999... so we'll need to round this
  //std::cout << c << '\n';

  //FixedPoint2 d(-5.01); // stored as -5.0099999... so we'll need to round this
  //std::cout << d << '\n';

	testAddition();

	FixedPoint2 a(-0.48);
	std::cout << a << '\n';

	std::cout << -a << '\n';

	std::cout << "Enter a number: "; // enter 5.678
	std::cin >> a;

	std::cout << "You entered: " << a << '\n';

  return 0;
}
