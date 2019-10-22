#include <iostream>

class Base {
  protected:
    int m_value;

  public:
    Base(int value) : m_value {value} {}
    int getValue() { return m_value; }
    virtual const char* getName() { return "Base"; }
};

class Derived : public Base {
  public:
    Derived(int value) : Base {value} {};
    const char* getName() { return "Derived"; }
};

class Animal {
  protected:
    std::string m_name;
    std::string m_sound {"???"};
    Animal(const std::string& name) : m_name{name} {}

  public:
    const std::string& getName() { return m_name; }
    std::string speak() { return m_sound; }
};

class Cat : public Animal {
  public:
    Cat(std::string name) : Animal{name} { m_sound = "meow"; }
};

class Dog : public Animal {
  public:
    Dog(std::string name) : Animal{name} { m_sound = "woof";}
};


void constfun(const Animal& an) {
  std::cout << "const is called!";
}
//void constfun(Animal& an) {
  //std::cout << "non-const is called!";
//}

int main() {
  Derived derived(5);
  std::cout << "derived is a " << derived.getName() << " and has value " << derived.getValue() << '\n';

  Derived &rDerived = derived;
  std::cout << "rDerived is a " << rDerived.getName() << " and has value " << rDerived.getValue() << '\n';

  Derived *pDerived = &derived;
  std::cout << "pDerived is a " << pDerived->getName() << " and has value " << pDerived->getValue() << '\n';

  Base& rbase = derived;
  std::cout << "rDerived is a " << rbase.getName() << " and has value " << rbase.getValue() << '\n';
  Base* pbase = &derived;
  std::cout << "rDerived is a " << pbase->getName() << " and has value " << pbase->getValue() << '\n';

  Cat fred("Fred"), misty("Misty"), zeke("Zeke");
  Dog garbo("Garbo"), pooky("Pooky"), truffle("Truffle");


  // Set up an array of pointers to animals, and set those pointers to our Cat and Dog objects
  Animal *animals[] = { &fred, &garbo, &misty, &pooky, &truffle, &zeke };
  for (auto animal : animals)
    std::cout << animal->getName() << " says " << animal->speak() << '\n';

  constfun(fred);
  return 0;
}
