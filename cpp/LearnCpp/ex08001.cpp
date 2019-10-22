#include <iostream>

class DateClass {
  public:
    int year, month, day;
    void print() {
      std::cout << year << "/" << month << "/" << day;
    }
};

class Employee {
  public:
    std::string name;
    int id;
    double wage;

    void print() {
      std::cout << "#" << id << ": " << name << " has wage " << wage << "\n";
    }

};

class Pair {
  public:
    int num1, num2;
    void set(const int& num1_, const int& num2_) {
      num1 = num1_;num2 = num2_;
    }
    void print() {
      std::cout << "Pair(" << num1 << ", " << num2 << ")\n";
    }
};

int main() {
  DateClass today {2019, 5, 17};
  today.print();
  std::cout << "\n";

  Employee alex { "Alex", 1, 25.00f };
  alex.print();

  Pair p1;
  p1.set(1, 1);
  Pair p2 {2, 2};

  p1.print();
  p2.print();

  return 0;
}
