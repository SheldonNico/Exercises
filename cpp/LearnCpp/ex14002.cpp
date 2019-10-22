#include <iostream>

#include <exception>
#include <string>

class Member {
  public:
    Member() {
      std::cerr << "Member allocated\n";
    }
    ~Member() {
      std::cout << "Member got destory\n";
    }
};

class A {
  private:
    int m_x;
    Member m_member;
    //Member* m_member;

  public:
    A(int x) : m_x{x}  {
      //m_member = new Member();
      if (x <= 0)
        throw "throw error";
    }
    ~A() {
      std::cout << "~A()\n";
    }
};

int main() {
  try {
    A a(0);
  } catch (const char* err) {
    std::cerr << "oops! got " << err <<"\n";
  }

  try {
    std::string s;
    s.resize(-1);
  } catch (std::exception & err) {
    std::cerr << "Standard exception: " << err.what() << "\n";
  }

  return 0;
}
