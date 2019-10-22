#include <iostream>
#include <string>

class Base {
  protected:
    int m_value;

  public:
    Base(int value) : m_value {value} {}

    friend std::ostream& operator<<(std::ostream &out, const Base& b) { return b.print(out); }
    virtual std::ostream& print(std::ostream& out) const {
      out << "Base";
      return out;
    }

    virtual ~Base() {}
};

class Derived : public Base {
  protected:
    std::string m_name;

  public:
    Derived(int value, std::string name) : Base {value}, m_name {name} {}
    const std::string& getName() { return m_name; }
    virtual std::ostream& print(std::ostream& out) const override {
      out << "Derived";
      return out;
    }
};

Base* getObject(bool bReturnedDerived) {
  if (bReturnedDerived)
    return new Derived {1, "Apple"};
  else
    return new Base {2};
}

int main() {
  Base *b = getObject(false);

  Derived* d = dynamic_cast<Derived*>(b);
  if (d)
    std::cout << "downcasting to get name: " << d->getName() << "\n";

  delete b;

  Base bp {0};
  std::cout << bp << "\n";
  Derived dp { 0, "dr" };
  std::cout << dp << "\n";

  Base& dpr = dp;
  std::cout << dpr << "\n";

  return 0;
}
