#include <iostream>

int main() {
  int value = 97;
  void *voidPtr = &value;

  std::cout << *static_cast<char*>(voidPtr) << "\n";
  voidPtr = nullptr;

  int* ptr = &value;
  std::cout << "one level pointer: " << *ptr << "\n";
  int** ptr_ptr = &ptr;
  std::cout << "two level pointer: " << **ptr_ptr << "\n";

  int** array = new int*[10];
  for (int count = 0; count < 10; ++count) {
    array[count] = new int[5];
  }

  // clean heap
  for (int count = 0; count < 10; ++count) {
    delete [] array[count];
  }
  std::cout << array[2][0] << "\n";
  delete [] array;

  return 0;
}
