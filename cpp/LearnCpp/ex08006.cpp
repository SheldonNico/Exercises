#include <iostream>
#include <cassert>

class IntArray {
  private:
    int* marray;
    int mlength;

  public:
    IntArray(int length) {
      assert(length > 0);
      marray = new int[length];
      mlength = length;
      std::cout << this << " construct Intarray\n";
    }
    ~IntArray() {
      std::cout << this << " destory Intarray\n";
      delete[] marray;
    }

    void setValue(int index, int value) {
      assert(index >= 0 && index < mlength);
      marray[index] = value;
    }
    int getValue(int index) {
      assert(index >= 0 && index < mlength);
      return marray[index];
    }
    int getLength() {
      return mlength;
    }
};

int main() {
  IntArray ar(10); // allocate 10 integers
  for (int count=0; count < 10; ++count)
    ar.setValue(count, count+1);

  std::cout << "The value of element 5 is: " << ar.getValue(5) << "\n";

  IntArray ar2(20); // allocate 10 integers
  std::cout << "The value of element 5 is: " << ar2.getValue(5) << "\n";

  return 0;
}
