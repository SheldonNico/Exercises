#include <iostream>
#include <vector>
#include <memory>

struct AbstractShape {
  virtual void Print() const = 0;
};

struct Square : public AbstractShape {
  void Print() const override { std::cout << "Square\n"; }
};

struct Triangle : public AbstractShape {
  void Print() const override { std::cout << "Triangle\n"; }
};

int main() {
  std::vector<std::unique_ptr<AbstractShape>> shapes;
  shapes.emplace_back(std::make_unique<Square>());
  auto tri = std::make_unique<Triangle>();
  shapes.emplace_back(std::move(tri));

  for (const auto& shape: shapes) { shape->Print(); }
  return 0;
}
