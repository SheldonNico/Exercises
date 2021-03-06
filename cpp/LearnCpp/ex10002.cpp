#include <cassert>
#include <initializer_list>
#include <iostream>

class IntArray {
  private:
    int m_length;
    int* m_data;

  public:
    IntArray() {}
    IntArray(int length) : m_length{length} {
      m_data = new int[length];
    }
    IntArray(const std::initializer_list<int> &list) : IntArray(static_cast<int>(list.size())) {
      int count = 0;
      for (const auto& ele: list) {
        m_data[count] = ele;
        ++count;
      }
    }

    ~IntArray() {
      delete [] m_data;
    }

    IntArray(const IntArray& list) {
      (*this) = list;
    }
    IntArray& operator= (const IntArray& list) {
      delete [] m_data;
      m_length = list.m_length;
      m_data = new int[m_length];
      for (int i {0}; i<m_length; ++i)
        m_data[i] = list.m_data[i];

      return *this;
    };

    int& operator[] (int index) {
      assert(index >= 0 && index < m_length);
      return m_data[index];
    }
    int getLength() { return m_length; }
};

int main() {
  IntArray array { {5, 4, 3, 2, 1} }; // initializer list
  for (int count = 0; count < array.getLength(); ++count)
    std::cout << array[count] << ' ';
  std::cout << "\n";

  array = { 1, 3, 5, 7, 9, 11 };
  IntArray array2(array);

  for (int count = 0; count < array2.getLength(); ++count)
    std::cout << array2[count] << ' ';
  std::cout << "\n";

  return 0;
}
