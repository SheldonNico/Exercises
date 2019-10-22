#include <iostream>
#include <string>
#include <cmath>

double mySqrt(double x) {
  if (x < 0.0)
    throw "Can not take sqrt of negative number.";

  return sqrt(x);
}

int main() {

  try {
    //throw -1;
    throw std::string {"string"};
  } catch (int x) {
    std::cerr << "we caught an in exception with value: " << x << "\n";
  } catch (double) {
    std::cerr << "Got exception of double!\n";
  } catch (const std::string& str) {
    std::cerr << "Got exception of string ref.\n";
  }

  //std::cout << "Enter a number: ";
  //double x;
  //std::cin >> x;

  //try {
    //double d = mySqrt(x);
    //std::cout << "The sqrt of " << x << " is " << d << "\n";
  //} catch (const char* exception) {
    //std::cerr << "Error: " << exception << "\n";

  //}
  try {
    std::cout << mySqrt(-1) << "\n";
  } catch (...) {
    std::cout << "got an exception of an undetermined type\n";
  }


  return 0;
}
