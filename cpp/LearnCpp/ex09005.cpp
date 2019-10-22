#include <iostream>
#include <cassert>

class Matrix {
  private:
    double data[4][4];

  public:
    Matrix() {
      for (int row{0}; row < 4; row++) {
        for (int col{0}; col < 4; col++) {
          data[row][col] = 0.0;
        }
      }
    }

    double& operator()(int row, int col);
    const double& operator()(int row, int col) const;
    void operator()();
};

double& Matrix::operator()(int row, int col) {
  assert(col >= 0 && col < 4);
  assert(row >= 0 && row < 4);
  return data[row][col];
}
const double& Matrix::operator()(int row, int col) const {
  assert(col >= 0 && col < 4);
  assert(row >= 0 && row < 4);
  return data[row][col];
}

void Matrix::operator()() {
  for (int row{0}; row < 4; row++) {
    for (int col{0}; col < 4; col++) {
      data[row][col] = 0.0;
    }
  }
}

class MyString {
  private:
    std::string m_string;

  public:
    MyString(std::string string = "") : m_string{string} {};

    std::string operator() (const int& start, const int& len) {
      return m_string.substr(start, len);
    }
    MyString operator+= (const MyString other) const {
      return {m_string + other.m_string};
    }
};



int main() {
  Matrix matrix;
  matrix(1, 3) = 4.5;
  std::cout << matrix(1, 3) << "\n";
  matrix();
  std::cout << matrix(1, 3) << "\n";

  MyString string{"Hello, world!"};
  std::cout << string(7, 5);

  return 0;
}
