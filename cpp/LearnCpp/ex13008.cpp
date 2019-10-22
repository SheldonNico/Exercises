#include <iostream>
#include <string>

template<typename T>
class Pair1 {
  protected:
    T m_first, m_second;

  public:
    Pair1(const T& first, const T& second) : m_first{first}, m_second{second} {}
    const T& first() const { return m_first; }
    const T& second() const { return m_second; }
    T& first() { return m_first; }
    T& second() { return m_second; }
};

template<typename T1, typename T2>
class Pair {
  protected:
    T1 m_first;
    T2 m_second;

  public:
    Pair() {}
    Pair(const T1& first, const T2& second) : m_first{first}, m_second{second} {}
    const T1& first() const { return m_first; }
    const T2& second() const { return m_second; }
    T1& first() { return m_first; }
    T2& second() { return m_second; }
};

template<typename T>
class StringValuePair : public Pair<std::string, T> {
  public:
    StringValuePair(const std::string& p1, const T& p2) : Pair<std::string, T>(p1, p2) { }
};


int main() {
  //Pair1<int> p1(5, 8);
  //std::cout << "Pair: " << p1.first() << ' ' << p1.second() << '\n';

  //const Pair1<double> p2(2.3, 4.5);
  //std::cout << "Pair: " << p2.first() << ' ' << p2.second() << '\n';

  Pair<int, double> p1(5, 6.7);
  std::cout << "Pair: " << p1.first() << ' ' << p1.second() << '\n';

  const Pair<double, int> p2(2.3, 4);
  std::cout << "Pair: " << p2.first() << ' ' << p2.second() << '\n';

  StringValuePair<int> svp("Hello", 5);
  std::cout << "Pair: " << svp.first() << ' ' << svp.second() << '\n';

  return 0;
}
