#include <iostream>
#include <chrono>
#include <array>
#include <algorithm>

class Something {
  public:
    Something() {
      ++count;
      id = count;
    }
    int getId() const {return id;}

    friend bool isEqual(const Something& s1, const Something& s2);

  private:
    static int count;
    int id;
};
int Something::count = 0;

bool isEqual(const Something& s1, const Something& s2) {
  return s1.id == s2.id;
}

class Fruit {
  public:
    enum FruitT {
      Apple, Banana, Cherry,
    };

  private:
    FruitT type;
    int percentage = 0;

  public:
    Fruit(FruitT type_) : type{type_} {}
    FruitT getType() { return type; }
    int getPct() { return percentage; }
};

class Timer {
  private:
    using clock_t = std::chrono::high_resolution_clock;
    using second_t = std::chrono::duration<double, std::ratio<1>>;
    std::chrono::time_point<clock_t> m_beg;

  public:
    Timer() : m_beg{clock_t::now()} {};

    void reset() {
      m_beg = clock_t::now();
    }

    double elpased() const {
      return std::chrono::duration_cast<second_t>(clock_t::now() - m_beg).count();
    }
};



int main() {
  const Something s1 {}, s2 {};
  std::cout << s1.getId() << " == " << s2.getId() << "\n";

  std::cout << isEqual(s1, s2) << "\n";

  Fruit apple(Fruit::Apple);

  if (apple.getType() == Fruit::Apple) {
    std::cout << "I am an apple\n";
  }

  const int greatElements = 10000;
  std::array<int, greatElements> array;
  for (int i {0}; i < greatElements; ++i) {
    array.at(i) = greatElements-1;
  }

  Timer timer;
  std::cout << "Time elpased: " << timer.elpased() << "seconds\n";

  timer.reset();
  std::sort(array.begin(), array.end());
  std::cout << "Time elpased: " << timer.elpased() << "seconds\n";

  return 0;
}
