#include <iostream>
#include <memory>
#include <vector>

struct A {
  int a = 10;
};

void func(const A& obj) {
  std::cout << "A member: a = " << obj.a << std::endl;
}

void func(const std::vector<int>& vec) {
  std::cout << "vector with size: " << vec.size() << std::endl;
}

int main() {
  auto p = std::make_unique<A>();
  std::cout << "Before move: " << p->a << std::endl;
  auto p_ = std::move(p);
  std::cout << "After move: " << p_->a << std::endl;

  func(*p_);
  func(std::vector<int>{1, 3, 4, 5, 6, 7});
}
