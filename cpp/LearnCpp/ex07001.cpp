#include <iostream>

void printElements(int (&arr)[4]) {
  int length { sizeof(arr) / sizeof(arr[0]) };
  for (int i {0}; i < length; i++) {
    std::cout << arr[i] << " ";
  }
  std::cout << "\n";
}

void foo(int* (&ptr)) {
  ptr = nullptr;
}

int main() {
  int arr[] {99, 20, 14, 80};
  printElements(arr);

  int x = 5;
  int *ptr = &x;
  std::cout << "ptr is: " << (ptr ? "non-null" : "null") << '\n'; // prints non-null
  foo(ptr);
  std::cout << "ptr is: " << (ptr ? "non-null" : "null") << '\n'; // prints null

  return 0;
}
