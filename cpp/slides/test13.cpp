#include <iostream>
template<typename T, typename S>
T awesome_func(const T& var_t, const S& var_s) {
  T result = var_t;
  std::cout << "Implicit type conversion: var_s is " << var_s << std::endl;
  return result;
}

template<typename T>
class TClass {
  public:
    TClass(const T& smth) : smth_(smth) {}
    void print() const {
      std::cout << smth_ << std::endl;
    }

  private:
    T smth_;
};


int main() {
  auto out = awesome_func<int, int>(int(5), char('c'));
  std::cout << out << std::endl;

  TClass<char> tv(100);
  tv.print();
  return 0;
}
