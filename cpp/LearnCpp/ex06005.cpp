#include <iostream>

std::string getName() {
  return "Alex";
}

int main() {
  int narray[5] {9, 7, 5};
  char carray[] = "Hello!";
  const char* name = "Alex";

  std::cout << narray << "\n";
  std::cout << carray << "\n";
  std::cout << name << "\n";

  char c = 'Q';
  std::cout << &c << "\n";

  //int array[10000000];

  int* ptr = new(std::nothrow) int(5);
  *ptr = 7;
  delete ptr;
  ptr = nullptr;
  // std::cout << *ptr << "\n";

  std::cout << "Enter a positive integer: ";
  int length;
  std::cin >> length;

  char* array = new char[length] { "Hello world" };
  std::cout << "I just allocated an array of integers of length " << length << "\n";
  array[0] = 98;
  std::cout << "first element: " << array[0] << "\n";
  delete[] array;


  return 0;
}
