#include <iostream>
#include <memory>

class Resource {
  public:
    Resource() { std::cout << "Resource acquired\n"; }
    ~Resource() { std::cout << "Resource destroyed\n"; }
};

int main() {
  auto ptr1 = std::make_shared<Resource>();
  {
    auto ptr2 = ptr1;
    std::cout << "ptr2 got killed.\n";
  }

  std::cout << "killing anogher pointer\n";


  return 0;
}
