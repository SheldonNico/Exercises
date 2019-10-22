#include <iostream>

class A {
  private:
    int m_x;

  public:
    A(const int& x) : m_x{x} {
      //if (x <= 0) {
        //throw 1;
      //}
    }
};

class B : public A {
  public:
    B(int x) try: A(x) {
      if (x <= 0) {
        throw 1;
      }
    }
    catch (...) {
      std::cerr << "Exception catched!\n";
    }
};


int main() {
  B b(0);

  return 0;
}
