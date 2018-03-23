#include <string>
#include <iostream>

int main() {
  const std::string hello = "Hello";
  const std::string message = hello +", world" +"!";

  std::cout << hello << std::endl;
  std::cout << message << std::endl;

  return 0;
}
