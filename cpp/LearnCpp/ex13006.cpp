#include <iostream>

template<typename T, int size>
class StaticArray_Base {
  protected:
    T m_array[size];

  public:
    T* getArray() { return m_array; }

    const T& operator[](const int& index) const {
      return  m_array[index];
    }
    T& operator[](const int& index) {
      return  m_array[index];
    }

    virtual void print() {
      for ( int i{0}; i < size; ++i ) {
        std::cout << m_array[i];
      }
      std::cout << "\n";
    }
};

template<typename T, int size>
class StaticArray: public StaticArray_Base<T, size> {
  public:
    StaticArray() {}
};


template<int size>
class StaticArray<double, size>: public StaticArray_Base<double, size> {
  public:
    StaticArray() {}

    virtual void print() override {
      for ( int i{0}; i < size; ++i ) {
        std::cout << this->m_array[i] << " ";
      }
      std::cout << "\n";

    }
};


int main() {

  // declare an integer array with room for 6 integers
  StaticArray<int, 6> intArray;

  // Fill it up in order, then print it backwards
  for (int count = 0; count < 6; ++count)
    intArray[count] = count;
  intArray.print();
  // declare a double buffer with room for 4 doubles
  StaticArray<double, 4> doubleArray;

  for (int count = 0; count < 4; ++count)
    doubleArray[count] = (4. + 0.1*count);
  doubleArray.print();

  return 0;
}
