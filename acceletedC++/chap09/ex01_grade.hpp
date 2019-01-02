#ifndef GUARD_grade_h
#define GUARD_grade_h
#include <vector>
#include <stdexcept>
#include <algorithm>

using std::domain_error;
using std::vector;

double grade(double, double, double);
double grade(double, double, const std::vector<double>&);

template <class T> T median(vector <T> arr) {
  typedef typename vector<T>::size_type vec_sz;

  vec_sz size = arr.size();
  if (size == 0) {
    throw domain_error("median of an empty array.");
  }

  std::sort(arr.begin(), arr.end());
  vec_sz mid = size/2;

  return size % 2 == 0 ? (arr[mid]+arr[mid-1])/2 : arr[mid];
}

#endif
