#include <iomanip>
#include <iostream>
#include <assert.h>
#include <cmath>

bool is_prime(const int& v) {
  if (v <= 1)
    return false;

  bool _prime = true;
  int max_split = std::sqrt(v);
  for (int i = 2; i <= max_split; ++i) {
    if (v % i == 0) {
      _prime = false;
      break;
    }
  }

  return _prime;
}

int main () {
  std::cout << "bool: " << sizeof(bool) << "bytes\n";
  std::cout << "char: " << sizeof(char) << "bytes\n";
  std::cout << "wchar_t: " << sizeof(wchar_t) << "bytes\n";
  std::cout << "char16_t: " << sizeof(char16_t) << "bytes\n";
  std::cout << "char32_t: " << sizeof(char32_t) << "bytes\n";
  std::cout << "short: " << sizeof(short) << "bytes\n";
  std::cout << "int: " << sizeof(int) << "bytes\n";
  std::cout << "long: " << sizeof(long) << "bytes\n";
  std::cout << "long long: " << sizeof(long long) << "bytes\n";
  std::cout << "float: " << sizeof(float) << "bytes\n";
  std::cout << "double: " << sizeof(double) << "bytes\n";
  std::cout << "long double: " << sizeof(long double) << "bytes\n";

  std::cout << "=================================\n";
  std::cout << "int8_t: " << sizeof(int8_t) << "bytes\n";
  std::cout << "uint8_t: " << sizeof(uint8_t) << "bytes\n";
  std::cout << "int16_t: " << sizeof(int16_t) << "bytes\n";
  std::cout << "uint16_t: " << sizeof(uint16_t) << "bytes\n";
  std::cout << "int32_t: " << sizeof(int32_t) << "bytes\n";
  std::cout << "uint32_t: " << sizeof(uint32_t) << "bytes\n";
  std::cout << "int64_t: " << sizeof(int64_t) << "bytes\n";
  std::cout << "uint64_t: " << sizeof(uint64_t) << "bytes\n";

  std::cout << std::boolalpha << true << " " << !true << "\n" << std::noboolalpha;
  std::cout << true << " " << !true << "\n";


  std::cout << "Please enter a integer:";
  int x { 0 };
  std::cin >> x;

  if (is_prime(x))
    std::cout << x << " is a prime\n";
  else
    std::cout << x << " is not a prime\n";

  return 0;
}
