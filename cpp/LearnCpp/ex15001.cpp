#include <iostream>

template<typename T>
class Auto_ptr {
  private:
    T* m_ptr;

  public:
    Auto_ptr(T* ptr=nullptr) : m_ptr{ptr} {}

    ~Auto_ptr() { delete m_ptr; };

    //Auto_ptr(Auto_ptr& a) {
      //m_ptr = a.m_ptr;
      //a.m_ptr = nullptr;
    //}
    //Auto_ptr& operator=(Auto_ptr& a) {
      //if (&a == this)
        //return *this;

      //delete m_ptr;
      //m_ptr = a.m_ptr;
      //a.m_ptr = nullptr;
      //return *this;
    //}

    Auto_ptr(const Auto_ptr& a) {
      m_ptr = new T;
      *m_ptr = *a.m_ptr;
    }
    Auto_ptr& operator=(const Auto_ptr& a) {
      if (&a == this)
        return *this;

      delete m_ptr;
      m_ptr = new T;
      *m_ptr = *a.m_ptr;
      return *this;
    }
    Auto_ptr(Auto_ptr&& a) {
      m_ptr = a.m_ptr;
      a.m_ptr = nullptr;
    }
    Auto_ptr& operator=(Auto_ptr&& a) {
      if (&a == this)
        return *this;

      delete m_ptr;
      m_ptr = a.m_ptr;
      a.m_ptr = nullptr;
      return *this;
    }


    T& operator*() const { return *m_ptr; }
    T* operator->() const { return m_ptr; }
    bool isNull() const { return m_ptr == nullptr; }
};

class Resource {
  public:
    Resource() { std::cout << "Resource created\n"; }
    ~Resource() { std::cout << "Resource destoryed\n"; }
};

Auto_ptr<Resource> func(int m) {
  Auto_ptr<Resource> ptr (new Resource);

  if (m <= 0)
    throw 1;

  std::cout << "function work as expect!\n";
  return ptr;
}

int main() {
  func(1);

  Auto_ptr<Resource> res1(new Resource);
  Auto_ptr<Resource> res2;

  std::cout << "res1 is " << (res1.isNull() ? "null\n" : "not null\n");
  std::cout << "res2 is " << (res2.isNull() ? "null\n" : "not null\n");

  res2 = res1;
  std::cout << "After transfer ownership.\n";
  std::cout << "res1 is " << (res1.isNull() ? "null\n" : "not null\n");
  std::cout << "res2 is " << (res2.isNull() ? "null\n" : "not null\n");

  std::cout << "test function copy\n";
  res2 = func(4);

  return 0;
}
