#include <algorithm>
#include <utility>
#include <iostream>

int main() {
  int x = 2, y = 4;
  std::cout << "Before swap x = " << x << ", y = " << y << "\n";

  std::swap(x, y);
  std::cout << "After swap x = " << x << ", y = " << y << "\n";

  /////////////////////////////////
  const int length = 5;
  int array[length] { 30, 50, 20, 10, 40 };

  std::cout << "Before sort: ";
  for (auto x : array)
    std::cout << x << " ";
  std::cout << "\n";


  //for (int startIndex = 0; startIndex < length-1; ++startIndex) {
    //int maxIndex = startIndex;
    //for (int i = startIndex+1; i < length; ++ i) {
      //if (array[maxIndex] < array[i])
        //maxIndex = i;
    //}
    //std::swap(array[startIndex], array[maxIndex]);
  //}

  bool notSorted = true;
  int count = 0;
  while (notSorted) {
    notSorted = false;
    for (int i = 0; i < length-1-count; ++i) {
      if (array[i] > array[i+1]) {
        std::swap(array[i], array[i+1]);
        notSorted = true;
      }
    }
    ++count;
  }

  std::cout << "After sort: ";
  for (auto x : array)
    std::cout << x << " ";
  std::cout << "\n";
  /////////////////////////////////
  int array_[length] { 30, 50, 20, 10, 40 };

  std::cout << "Before sort: ";
  for (auto x : array_)
    std::cout << x << " ";
  std::cout << "\n";

  std::sort(array_, array_+length);

  std::cout << "After sort: ";
  for (auto x : array_)
    std::cout << x << " ";
  std::cout << "\n";



  return 0;
}
