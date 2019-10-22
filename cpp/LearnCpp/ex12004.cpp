#include <iostream>
#include <vector>
#include <functional>

class PoweredDevice {
  public:
    PoweredDevice(int power) {
      std::cout << "PoweredDevice creted: " << power << "\n";
    }
    virtual void printName() { std::cout << "I'm powereddevice."; }
};

class Scanner : virtual public PoweredDevice {
  public:
    Scanner(int scanner, int power) : PoweredDevice{power} {
      std::cout << "Scanner created: " << scanner << "\n";
    }
    void printName() { std::cout << "I'm scanner."; }
};
class Printer : virtual public PoweredDevice {
  public:
    Printer(int printer, int power) : PoweredDevice{power} {
      std::cout << "Printer created: " << printer << "\n";
    }
    void printName() { std::cout << "I'm printer."; }
};

class Copier : public Scanner, public Printer {
  public:
    Copier(int scanner, int printer, int power) :
      PoweredDevice { power },
      Scanner {scanner, power}, Printer {printer, power} {}
    void printName() { std::cout << "I'm copier."; }
};


int main() {
  Copier c {1, 2, 3};

  std::vector<PoweredDevice*> vec;
  vec.push_back(new Copier(1, 2, 1));
  vec.push_back(new PoweredDevice(1));

  std::cout << "Vector of PoweredDevice*\n";
  for (const auto& e : vec) {
    e->printName();
    std::cout << "\n";
  }

  std::vector< std::reference_wrapper<PoweredDevice> > vec2;
  PoweredDevice p1 {0};
  Printer p2 {1, 1};
  std::cout << "Vector of PoweredDevice.ref\n";
  vec2.push_back( p1 );
  vec2.push_back( p2 );
  for (const auto& e : vec2) {
    e.get().printName();
    std::cout << "\n";
  }

  return 0;
}
