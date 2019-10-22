#include <iostream>
#include <chrono>

template <typename T>
class DynamicArray {
  private:
    T* m_array;
    int m_length;

  public:
    DynamicArray(int length) : m_length{length} {
      m_array = new T[length];
    }
    ~DynamicArray() {
      delete [] m_array;
    }

    // Copy
    //DynamicArray(const DynamicArray& arr) : m_length{arr.m_length} {
      //m_array = new T[m_length];
      //for (int i{0}; i < m_length; ++i)
        //m_array[i] = arr.m_array[i];
    //}
    //DynamicArray& operator=(const DynamicArray& arr) {
      //delete m_array;
      //m_length = arr.m_length;

      //m_array = new T[m_length];
      //for (int i{0}; i < m_length; ++i)
        //m_array[i] = arr.m_array[i];

      //return *this;
    //}

    // Move
    DynamicArray(DynamicArray&& arr) : m_length{arr.m_length} {
      m_array = arr.m_array;
      arr.m_array = nullptr;
      arr.m_length = 0;
    }
    DynamicArray& operator=(DynamicArray&& arr) {
      delete m_array;
      m_length = arr.m_length;
      m_array = arr.m_array;
      arr.m_array = nullptr;
      arr.m_length = 0;

      return *this;
    }


    const int getLength() const { return m_length; }
    T& operator[](const int& index) {
      return m_array[index];
    }
    const T& operator[](const int& index) const {
      return m_array[index];
    }
};

class Timer {
  private:
    using clock_t = std::chrono::high_resolution_clock;
    using second_t = std::chrono::duration<double, std::ratio<1> >;

    std::chrono::time_point<clock_t> m_beg;

  public:
    Timer() : m_beg{clock_t::now()} {}
    void reset() {m_beg = clock_t::now();}
    double elapsed() const {
      return std::chrono::duration_cast<second_t>(clock_t::now() - m_beg).count();
    }
};

// Return a copy of arr with all of the values doubled
DynamicArray<int> cloneArrayAndDouble(const DynamicArray<int> &arr)
{
	DynamicArray<int> dbl(arr.getLength());
	for (int i = 0; i < arr.getLength(); ++i)
		dbl[i] = arr[i] * 2;

	return dbl;
}

int main() {

  Timer t;

  DynamicArray<int> arr(1000000);

  for (int i = 0; i < arr.getLength(); i++)
    arr[i] = i;

  arr = cloneArrayAndDouble(arr);

  std::cout << t.elapsed();
  return 0;
}
