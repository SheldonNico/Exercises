#include <iostream>
#include <vector>
#include <cmath>
#include <limits>

void countDown(int count) {
  std::cout << "push " << count << "\n";

  if (count > 1)
    countDown(count - 1);

  std::cout << "pop " << count << "\n";
}

int fibonacci(int num) {
  if (num == 0) {
    return 0;
  } else if (num == 1) {
    return 1;
  } else if (num > 0) {
    return fibonacci(num -1) + fibonacci(num - 2);
  } else {
    return 0;
  }
}

int factorial(const int& num) {
  if (num > 0) {
    return num * factorial(num-1);
  } else {
    return 1;
  }
}

int sumDigit(const int& num) {
  std::string digits = std::to_string(num);
  int out = 0;
  for (auto d : digits) {
    out += static_cast<int>(d - '0');
  }
  return out;
}

void printBin(const int& num) {
  if (num < 0) {
    int num_= num + std::numeric_limits<int>::max() + 1;
    printBin(num_);
  } else if (num < 2) {
    std::cout << num;
  } else {
    printBin(num / 2);
    std::cout << num % 2;
  }

}

int main() {
  std::vector<int> vec { 0, 1, 2 };
  vec.resize(4);

  std::cout << "The length is: " << vec.size() << "\n";
  std::cout << "The capicity is: " << vec.capacity() << "\n";

  for (const auto & element : vec)
    std::cout << element << " ";
  std::cout << "\n";


  countDown(5);

  for (int i {0}; i < 10; ++i) {
    std::cout << "fib " << i << " = " << fibonacci(i) << "\n";
  }
  for (int i {0}; i < 10; ++i) {
    std::cout << "fac " << i << " = " << factorial(i) << "\n";
  }
  std::cout << sumDigit(93427) << "\n";

  printBin(-15);
  std::cout << "\n";

  return 0;
}
