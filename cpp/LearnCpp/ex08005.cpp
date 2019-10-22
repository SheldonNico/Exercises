#include <iostream>
#include <string>
#include <cstdint>

class Fraction {
  private:
    int numerator, denominator;

  public:
    Fraction() { numerator = 0; denominator = 1; }

    int getNumerator() { return numerator; }
    int getDenominator() { return denominator; }
    double getValue() { return static_cast<double>(numerator) / denominator; }
};

class Date1 {
  private:
    int year, month, day;
  public:
    void print() {
      std::cout << year << "\n";
    }
};

class Date2 {
  private:
    int year, month, day;
  public:
    void print() {
      std::cout << year << "\n";
    }
    Date2(const int& year_, const int& month_, const int& day_) {
      year = year_; month = month_; day = day_;
    }
};

class Ball {
  private:
    std::string color;
    double radius;

  public:
    Ball() { color = "red"; radius = 0; }
    Ball(std::string c) {
      color = c; radius = 0;
    }
    Ball(double r) {
      radius = r; color = "red";
    }

    Ball(std::string c, double r) {
      color = c; radius = r;
    }

    void print() {
      std::cout << "Ball with c(" << color << ") radius(" << radius << ")\n";
    }
};

class RGBA {
  private:
    std::uint_fast8_t red {0}, green {0}, blue {0}, alpha {255};

  public:
    RGBA() {};
    RGBA(std::uint_fast8_t r, std::uint_fast8_t g, std::uint_fast8_t b, std::uint_fast8_t a) :
      red{r}, green{g}, blue{b}, alpha{a} {};
    void print() {
      std::cout << "r=" << static_cast<int>(red) << " g=" << static_cast<int>(green) << " b=" << static_cast<int>(blue) << " a=" << static_cast<int>(alpha) << "\n";
    }
};

int main() {
  Fraction frac ;
  std::cout << frac.getNumerator();

  Date1 dt {};
  dt.print();

  Date2 dt2 {2019, 4, 5};
  dt2.print();

  Ball def;
  def.print();

  Ball blue("blue");
  blue.print();

  Ball twenty(20.0);
  twenty.print();

  Ball blueTwenty("blue", 20.0);
  blueTwenty.print();

  RGBA teal(0, 127, 127, 255);
  teal.print();
  return 0;
}
