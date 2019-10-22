#include <iostream>
using std::cout;
using std::endl;

struct A {
  int a = 10;
};

int main() {
  A a;
  cout << a.a << endl;
  A* a_ptr = &a;
  A& a_ref = a;
  cout << a_ptr->a << " ptr: " << (*a_ptr).a  << " ref: " << a_ref.a << endl;
  a_ref.a = 12;
  cout << a_ptr->a << " ptr: " << (*a_ptr).a  << " ref: " << a_ref.a << endl;
}
