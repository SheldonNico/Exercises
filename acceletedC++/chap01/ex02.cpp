#include <iostream>
#include <string>

int main() {
  {
    const std::string s = "a string";
    std::cout << s << std::endl;
  }

  {
    const std::string s = "another string";
    std::cout << s << std::endl;
  }

  {
    const std::string s = "a string";
    std::cout << s << std::endl;

    {
      const std::string s = "another string";
      std::cout << s << std::endl;
    }
  }

  {
    std::string s = "a string";
    {
      std::string x = s + ", really";
      // std::cout << s << std::endl;
      std::cout << x << std::endl;
      std::cout << s << std::endl;
    }
    // std::cout << x << std::endl;
    std::cout << s << std::endl;
  }

  return 0;
}
