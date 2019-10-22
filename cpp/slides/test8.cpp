#include <iostream>
#include <memory>

struct A {
  A(int a) : _a(a) { std::cout << "I'm alive!\n"; }
  ~A() { std::cout << "I'm dead...\n"; }

  private:
    int _a;
};

int main() {
  auto a_ptr = std::make_shared<A>(10);
  std::cout << "count of ptr: " << a_ptr.use_count() << std::endl;

  // auto b_ptr = a_ptr;
  // std::cout << "after copy assign count of ptr: " << b_ptr.use_count() << std::endl;

  {
    std::shared_ptr<A> b_ptr = a_ptr;
    std::cout << "after copy assign count of ptr: " << a_ptr.use_count() << std::endl;
  }

  {
    std::shared_ptr<A> b_ptr = std::move(a_ptr);
    std::cout << "after move assign count of ptr: " << b_ptr.use_count() << std::endl;
  }

  std::cout << "after move assign count of ptr: " << a_ptr.use_count() << std::endl;
  return 0;
}
