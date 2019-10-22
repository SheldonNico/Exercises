#include <iostream>
#include <string>

class Animal {
  protected:
    std::string m_name;

  public:
    Animal(std::string name) : m_name{name} {}

    std::string getName() { return m_name; }
    virtual const char* speak() = 0;
};
const char* Animal::speak() {
  return "???";
}

class Dragonfly final: public Animal {
  public:
    Dragonfly(std::string name) : Animal { name } {}
    const char* speak()  { return Animal::speak(); }
};



int main() {
  Dragonfly dfly("Sally");

  std::cout << dfly.getName() << " says " << dfly.speak() << "\n";

  return 0;
}
