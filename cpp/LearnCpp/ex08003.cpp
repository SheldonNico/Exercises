#include <iostream>
#include <cassert>

class Stack {
  private:
    static const int maxStackLength { 10 };
    int array[maxStackLength];
    int length {0};

  public:
    void print() {
      std::cout << "( ";
      for (int count {0}; count < length; ++count) {
        std::cout << array[count] << " ";
      }
      std::cout << ")\n";
    }

    void reset() {
      length = 0;
    }

    bool push(int value) {
      if (length == maxStackLength)
        return false;

      array[length] = value;
      ++length;

      return true;
    }

    int pop() {
      assert( length > 0 && "can not pop empty stack" );
      --length;

      return array[length+1];
    }
};

int main() {
  Stack stack;

  stack.reset();
  stack.print();

  stack.push(5);
  stack.push(3);
  stack.push(8);
  stack.print();

  stack.pop();
  stack.print();

  stack.pop();
  stack.pop();

  stack.print();


  return 0;
}
