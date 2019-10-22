#include <iostream>
#include <tuple>
#include <string>

struct Student {
  int v = 1;
};

void foo(int* ptr) {
  *ptr = 6;
}

void foo2(Student*& ptr) {
  ptr = new Student();
  ptr->v = 10;
  std::cout << "inside function v is " << ptr->v << "\n";
}

int* doubleValue(int x) {
  int value = x*2;
  return &value;
}

std::tuple<int, std::string> returnTuple() {
  return std::make_tuple(5, "more and more");
}

int& doubleValue2(int x) {
  int value = x * 2;
  return value;
}

const int& returnByReference() {
  return 5;
}

int sumTo(const int& num) {
  int sum = 0;
  for (int count{1}; count <= num; ++count)
    sum += count;

  return sum;
}

void printEmployeeName(const Student& st) {
}

void minmax(const int& num1, const int& num2, int& bigger, int& smaller) {
  if (num1 < num2) {
    bigger = num2;
    smaller = num1;
  } else {
    bigger = num1;
    smaller = num2;
  }
}

int getIndexOfLargestValue(const int* arr[], const int& size) {
}

const int& getElement(const int* arr[], const int& ind) {
}

inline int min(int x, int y) {
  return x > y ? x : y;
}

int main() {
  int value = 5;
  std::cout << "Before function call: " << value << "\n";
  foo(&value);
  std::cout << "After pass ptr : " << value << "\n";

  Student st {};
  Student* st_ptr = &st;
  std::cout << "Before function call: " << st.v << "\n";
  foo2(st_ptr);
  std::cout << "After pass ptr : " << st.v << "\n";
  std::cout << "After pass ptr : " << st_ptr->v << "\n";

  int* r_ptr = doubleValue(5);
  // std::cout << "after destory " << *r_ptr;

  //const int& ref = returnByReference();
  //std::cout << ref;

  auto s = returnTuple();
  std::cout << std::get<1>(s) << "\n";
  int t1; std::string t2;
  std::tie(t1, t2) = returnTuple();

  auto [t1_, t2_] = returnTuple();
  std::cout << t2_ << "\n";


  return 0;
}
