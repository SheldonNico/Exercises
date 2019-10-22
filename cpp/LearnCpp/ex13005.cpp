#include <iostream>
#include <initializer_list>

template <typename T, int size>
class StaticArray {
  private:
    T m_array[size];

  public:
    StaticArray(const std::initializer_list<T>& li) {
      int count = 0;
      for (const auto& v : li) {
        if (count >= size) break;
        m_array[count] = v;
        count += 1;
      }
    }
    T& operator[](const int& index) {
      return m_array[index];
    }
    const T& operator[](const int& index) const {
      return m_array[index];
    }
};

template<typename T, int size>
void print(const StaticArray<T, size>& array) {
  for (int count {0}; count < size; count++) {
    std::cout << array[count] << " ";
  }
  std::cout << "\n";
}

template<int size>
void print(const StaticArray<char, size>& array) {
  for (int count {0}; count < size; count++) {
    std::cout << array[count];
  }
  std::cout << "\n";
}


int main() {
  StaticArray<int, 4> int4 { 0, 1, 2, 3 };
  print(int4);

  StaticArray<char, 14> char14 { 'h', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '!' };
  print(char14);

  return 0;
}
