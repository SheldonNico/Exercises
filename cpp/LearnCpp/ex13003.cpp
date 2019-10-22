#include <iostream>
#include <initializer_list>

template <typename T, int size>
class StaticArray {
  private:
    T m_array[size];

  public:
    StaticArray(const std::initializer_list<T> & li) {
      int count {0};
      for (const auto& v : li) {
        m_array[count] = v;
        count += 1;
      }
    }
    T* getArray();
    T& operator[] (const int& index) {
      return m_array[index];
    }
};

int main() {
  StaticArray<int, 4> arr {0, 1, 3, 4};

  for (int i{0}; i < 4; ++i) {
    std::cout << "#" << i << ": " << arr[i] / 2 << "\n";
  }

  StaticArray<double, 4> arr2 {0, 1, 3, 4};

  for (int i{0}; i < 4; ++i) {
    std::cout << "#" << i << ": " << arr2[i] / 2 << "\n";
  }
  return 0;
}
