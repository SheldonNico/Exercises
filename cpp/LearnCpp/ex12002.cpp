#include <iostream>

class Base {
  public:
    virtual ~Base() {
      std::cout << "Calling ~Base()\n";
    }

    virtual const char* getName() const { return "Base"; };
};

class Derived: public Base {
  private:
    int* m_array;

  public:
    Derived(int len) {
      m_array = new int[len];
    }
    virtual ~Derived() {
      std::cout << "Calling ~Derivied()\n";
      delete [] m_array;
    }

    virtual const char* getName() const override { return "Derived"; };

};

int main() {
  Derived* derived = new Derived(5);
  Base* base = derived;


  std::cout << base->Base::getName() << "\n";
  std::cout << base->getName() << "\n";
  delete base;

  return 0;
}
