#include <vector>
#include <iostream>
#include <stdexcept>

using std::cout;
using std::endl;
using std::vector;
using std::domain_error;

double average(const vector<double>& vec) {
  if (vec.size() == 0) {
    throw domain_error("divided by 0");
  } else {
    double sum = 0;
    for (int i = 0; i < vec.size(); ++i) {
        sum += vec[i];
    }

    return sum / vec.size();
  }
}

int main() {
  vector<double> vec;
  vec.push_back(1);
  vec.push_back(2);
  cout << average(vec) << endl;
  return 0;
}
