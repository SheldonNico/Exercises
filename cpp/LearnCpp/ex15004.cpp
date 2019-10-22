#include <iostream>
#include <utility>
#include <memory>
#include <vector>

template<typename T>
void myswap(T& a, T& b) {
  T tmp { std::move(a) }; // copy construct
  a = std::move(b); // copy assign
  b = std::move(tmp); // copy assign
}

class Resource {
  public:
    Resource() { std::cout << "Resource acquired.\n"; };
    ~Resource() { std::cout << "Resource destoryed.\n"; };

    friend std::ostream& operator<< (std::ostream& os, const Resource& res) {
      (void) res;
      os << "I am a resource";
      return os;
    }
};

class Fraction {
  private:
    int m_numerator = 0, m_denominator= 1;

  public:
    Fraction(const int& numerator = 0, const int& denominator = 1) : m_numerator{numerator}, m_denominator{denominator} {}

    friend std::ostream& operator<<(std::ostream& os, const Fraction& frac) {
      os << frac.m_numerator << "/" << frac.m_denominator;
      return os;
    }
};

std::unique_ptr<Resource> createResource() {
  return std::make_unique<Resource>();
}

void takeOwnerShip(std::unique_ptr<Resource> res) {
  if (res)
    std::cout << *res << " with ownership has been transferred.\n";
}

int main() {
  std::string x {"abc"};
  std::string y {"de"};

  std::cout << "x: " << x << "\n";
  std::cout << "y: " << y << "\n";

  myswap(x, y);

  std::cout << "x: " << x << "\n";
  std::cout << "y: " << y << "\n";

  auto res1 = std::make_unique<Resource>();
  std::unique_ptr<Resource> res2;

  std::cout << "res1 is " << (static_cast<bool>(res1) ? "not null\n" : "null\n");
  std::cout << "res2 is " << (static_cast<bool>(res2) ? "not null\n" : "null\n");
  res2 = std::move(res1);

  std::cout << "ownership transferred: \n";
  std::cout << "res1 is " << (static_cast<bool>(res1) ? "not null\n" : "null\n");
  std::cout << "res2 is " << (static_cast<bool>(res2) ? "not null\n" : "null\n");

  if (res2)
    std::cout << (*res2) << "\n";

  auto f1 = std::make_unique<Fraction[]>(4);
  std::cout << f1[0] << "\n";

  std::unique_ptr<Resource> ptr = createResource();

  takeOwnerShip(std::move(res2));


  return 0;
}
