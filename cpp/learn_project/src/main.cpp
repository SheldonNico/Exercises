#define _USE_MATH_DEFINES

#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <fstream>
#include <cmath>
#include "hello.h"
#include "blah.h"

class Blah {
public:
  void someFunc() const { std::cout << "const" << std::endl; }
  void someFunc() { std::cout << "non const" << std::endl; }
};

void printv(const std::string& str) {
  std::cout << "lvalue: " << str << std::endl;
}

void printv(std::string&& str) {
  std::cout << "rvalue: " << str << std::endl;
}


class Printer {
public:
  void printv(const std::string& str) {
    // some_str = str; // copy the str
    std::cout << "lvalue set in Printer: " << str << std::endl;
  }
  void printv(const std::string&& str) {
    // just use the str, since str is a rvalue, it is moved only in this case.
    // This is faster
    some_str = std::move(str);
    std::cout << "rvalue set in Printer: " << str << std::endl;
  }
private:
  std::string some_str;
};

class Human {
public:
  Human(int kindness) : kindness_(kindness) {} ;
  // why h2's private member can be access?
  // private is not a condition in instances. It's in class level
  // The fundamental point of encapsulation and data hiding isn't taking away access rights.
  // Given this flexibility won't break any other classes. So why not?
  // This can only definded inside class? is it possible to make < operator outside class definition?
  bool operator<(const Human& h2) const {
    return kindness_ < h2.kindness_;
  }
private:
  int kindness_;
};

class Rectangle {
public:
  Rectangle(int w, int h) : width_{w}, height_{h} {}
  virtual void print() const {
    std::cout << "Rect: " << width_ << " " << height_ << std::endl;
  }
  int width() const { return width_; }
  int height() const { return height_;  }

protected:
  int width_= 0, height_ = 0;
};

class Square : public Rectangle {
public:
  explicit Square(int size) : Rectangle{size, size} {}

  void print() const override {
    std::cout << "Square: " << width_ << " " << height_ << std::endl;
  }

  void square_fun() const {
    std::cout << "new function only in square is called!" << std::endl;
  }
};

class Point {
public:
  Point(int x, int y) : x_{x}, y_{y} {}
  float Distance(const Point& other) const {
    float x_diff = x_ - other.x_;
    float y_diff = y_ - other.y_;
    return std::sqrt(x_*x_ + y_*y_);
  }

  static float Distance(const Point& a, const Point& b) {
    return a.Distance(b);
  }

private:
  int x_ = 0, y_ = 0;
};

int main() {
  // const ref to class
  Blah blah = {};
  blah.someFunc();

  const Blah& blah_ref = blah;
  blah_ref.someFunc();

  print_hello();
  print_blah();

  // move semantics
  std::string hello = "Hello";
  printv(hello);
  printv("hello");
  printv(std::move(hello));

  // inside class
  Printer printer = {};
  printer.printv(hello);
  printer.printv("Hi");
  printer.printv(std::move(hello));
  // This is forbiden, if hello is moved, there is nothing left in hello
  std::cout << "after moved: " << hello << std::endl;

  // operators
  std::vector<Human> humans = {Human{20}, Human{0}, Human{10}};
  std::sort(humans.begin(), humans.end());

  // inheritance
  Square sq(10);
  std::cout << sq.width() << " " << sq.height() << std::endl;
  sq.print();
  const Rectangle& sq_ref = sq; // cast back to rectangle, this is inversable
  sq_ref.print(); // compiler use type to decide which print to use:

  std::ifstream f_in("test.log", std::ios_base::in);

  // static class method
  Point p1 = {1, 1}, p2 = {2, 2};
  std::cout << "Distance: " << p1.Distance(p2) << std::endl;
  std::cout << "Yet another distance: " << Point::Distance(p1, p2) << std::endl;

  // numbers
  std::cout << "size of int: " << sizeof(int) << std::endl;
  std::cout << "size of double: " << sizeof(double) << std::endl;

  // precisions
  float base = 1000000;
  std::cout << std::setprecision(20) << "pi is: " << M_PI << " sum of pi and 1000000 is: " << base + M_PI << std::endl;

  // pointer
  int a = 42;
  int* a_ptr = &a;
  int b = *a_ptr;
  int& c = *a_ptr;
  std::cout<< "a = " << a << " a_ptr = " << a_ptr << " b = " << b << " c = " << c << std::endl;
  *a_ptr = 13;
  std::cout<< "a = " << a << " a_ptr = " << a_ptr << " b = " << b << " c = " << c << std::endl;

  int ar[] = {1, 2, 3};
  for (int i=0; i < 6; i++) {
    std::cout << i << ": value: " << ar[i] << ": address: " << &ar[i] << std::endl;
  }

  // Polymorphism by pointer
  std::vector<Rectangle*> shapes;
  Square psq(4);
  Rectangle prect(5, 7);
  shapes.push_back(&psq);
  shapes.push_back(&prect);
  for (auto p : shapes)
    p->print();

  std::vector<Rectangle> shapesv;
  shapesv.push_back(psq);
  shapesv.push_back(prect);
  for (auto p : shapesv)
    p.print();

  return 0;
}
