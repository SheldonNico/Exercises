#include <iostream>
#include <cassert>

template <typename T>
const T& max(const T& x, const T& y) {
  return x > y ? x : y;
}

template <class T>
T average(T *array, int length) {
  T sum {0};
  for (int count{0}; count < length; ++count) {
    sum += array[count];
  }
  return sum / length;
}

class Cents {
  private:
    int m_cents;

  public:
    Cents (int cents) : m_cents {cents} {}

    friend bool operator>(const Cents& c1, const Cents& c2) {
      return c1.m_cents > c2.m_cents;
    }

    friend std::ostream& operator<<(std::ostream& os, const Cents& c) {
      os << "Cents: " << c.m_cents;
      return os;
    }

    Cents& operator+=(Cents c) {
      m_cents += c.m_cents;
      return *this;
    }
    Cents& operator/(Cents c) {
      m_cents /= c.m_cents;
      return *this;
    }
};

template <typename T>
class Array {
  private:
    int m_length;
    T *m_data;

  public:
    Array() : m_length {0}, m_data { nullptr } {}
    Array(int length) {
      m_data = new T[length];
      m_length = length;
    }
    ~Array() {
      delete [] m_data;
    }

    void erase() {
      delete [] m_data;
      m_data = nullptr;
      m_length = 0;
    }

    T& operator[] (const int index) {
      assert( index >= 0 && index < m_length );
      return m_data[index];
    }
    int getLength();
};
template <typename T>
int Array<T>::getLength() {
  return m_length;
}

int main() {
  int i = max(3, 7); // returns 7
  std::cout << i << '\n';

  double d = max(6.34, 18.523); // returns 18.523
  std::cout << d << '\n';

  char ch = max('a', '6'); // returns 'a'
  std::cout << ch << '\n';

  std::cout << max(Cents{5}, Cents {6}) << "\n";

  int array1[] = { 5, 3, 2, 1, 4 };
  std::cout << average(array1, 5) << '\n';

  double array2[] = { 3.12, 3.45, 9.23, 6.34 };
  std::cout << average(array2, 4) << '\n';

  Cents array3[] = { Cents(5), Cents(10), Cents(15), Cents(14) };
  std::cout << average(array3, 4) << '\n';


  Array<int> intArray(12);
  Array<double> doubleArray(12);

  for (int count = 0; count < intArray.getLength(); ++count)
  {
    intArray[count] = count;
    doubleArray[count] = count + 0.5;
  }

  for (int count = intArray.getLength()-1; count >= 0; --count)
    std::cout << "#" << count << ": "<< intArray[count] << "\t" << doubleArray[count] << '\n';

  return 0;
}
