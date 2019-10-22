#include <iostream>
#include <string>

class Base {
  public:
    const int m_id;
    Base(int id=0) : m_id{id} {
      std::cout << "Base constructor\n";
    }

    int getId() const { return m_id; }
};

class Derived : public Base {
  public:
    double m_cost;
    Derived(double cost=0.0, int id=0) : Base {id}, m_cost {cost} {
      std::cout << "Derived constructor\n";
    }
    double getCost() const { return m_cost; }
};

class Person {
  private:
    std::string m_name;
    unsigned int m_age;

  public:
    Person(std::string name = "", unsigned int age = 0) :
      m_name {name}, m_age {age} { }
    std::string getName() { return m_name; }
    unsigned int getAge() { return m_age; }
};

class BaseballPlayer : public Person {
  private:
    double m_battingAverage;
    int m_homeRuns;

  public:
    BaseballPlayer(std::string name="", unsigned int age=0,
        double battingAverage=0, int homeRuns = 0 ) :
      Person {name, age}, m_battingAverage {battingAverage}, m_homeRuns {homeRuns} {}

    double getBattingAverage() { return m_battingAverage; }
    int getHomeRuns() { return m_homeRuns; }
};

class Fruit {
  protected:
    std::string m_name, m_color;

  public:
    Fruit(std::string name, std::string color) : m_name {name}, m_color {color} {}
};

class Apple : public Fruit {
  private:
    double m_fiber;

  public:
    Apple(std::string name, std::string color, double fiber) :
      Fruit {name, color}, m_fiber {fiber} {}
    friend std::ostream& operator<< (std::ostream& os, const Apple& fruit) {
      os << "Apple(" << fruit.m_name << ", " << fruit.m_color << ", " << fruit.m_fiber << ")";
      return os;
    }
};


int main() {

  Derived derived {11.4, 3};

  std::cout << "id is parent's public constants: " << derived.getId() << "\n";

  BaseballPlayer pedro("Pedro Cerrano", 32, 0.342, 42);

  std::cout << pedro.getName() << '\n';
  std::cout << pedro.getAge() << '\n';
  std::cout << pedro.getHomeRuns() << '\n';

  std::cout << Apple {"Red delicious", "red", 4.3} << "\n";
  const Apple a("Red delicious", "red", 4.2);
  std::cout << a << "\n";
  return 0;
}
