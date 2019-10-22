#include <string>
#include <iostream>

#include <cassert>
#include <cstring>

class MyString {
  private:
    char* m_data;
    int m_length;

  public:
    MyString(const char* source="") {
      assert(source);
      m_length = std::strlen(source) + 1;

      m_data = new char[m_length];
      for (int i {0}; i < m_length; ++i)
        m_data[i] = source[i];
      m_data[m_length-1] = '\0';
    }

    MyString(const MyString& source) {
      m_data = new char[source.m_length];
      for (int i {0}; i < m_length; ++i)
        m_data[i] = source.m_data[i];
      m_data[m_length-1] = '\0';
      m_length = source.m_length;
    }

    MyString& operator= (const MyString& source) {
      if (this != &source) {
        delete [] m_data;

        m_data = new char[source.m_length];
        for (int i {0}; i < m_length; ++i)
          m_data[i] = source.m_data[i];
        m_data[m_length-1] = '\0';
        m_length = source.m_length;
      }

      return *this;
    }

    ~MyString() {
      std::cout << "Got destoried\n";
      delete[] m_data;
    }

    char* getString() {return m_data;}
    int getlength() {return m_length;}
};

int main() {
  MyString hello {"Hello, world!"};
  {
    MyString copy;
    copy = hello;

    //MyString copy = hello;
  }

  std::cout << "What hello contained: " << hello.getString() << "\n";
  return 0;
}
