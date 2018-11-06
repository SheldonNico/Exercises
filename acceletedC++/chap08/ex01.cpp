#include <stdexcept>
#include <vector>
#include <iostream>
#include <iterator>
#include <ctype.h>
#include <algorithm>
#include <numeric>

#include "Student.h"
#include "median.h"

using namespace std;

bool did_all_hw(const Student& s) {
  return (find(s.homework.begin(), s.homework.end(), 0) == s.homework.end());
}

double median_grade(const Student& s) {
  try {
    return grade(s);
  } catch (domain_error) {
    return grade(s.midterm, s.final, 0);
  }
}

double average(const vector<double>& v) {
  return accumulate(v.begin(), v.end(), 0.0) / v.size();
}

double average_grade(const Student& s) {
  return grade(s.midterm, s.final, average(s.homework));
}

// median of the nonzero elements of `s.homework', or `0' if no such elements exist
double optimistic_median_grade(const Student& s) {
  vector<double> nonzero;
  remove_copy(s.homework.begin(), s.homework.end(), back_inserter(nonzero), 0);

  if (nonzero.empty())
    return grade(s.median, s.final, 0);
  else
    return grade(s.median, s.final, nonzero);
}

template <class Function>
double analysis(const vector<Student> students, Function grading_func) {
  vector<double> grades;

  transform(students.begin(), students.end(), back_inserter(grades), grading_func);
  return median(grades);
}

template <class Function>
void write_analysis(ostream& out, const string& name, Function grading_func,
                    const vector<Student>& did, const vector<Student>& didnt) {
  out << name << ": median(did) = " << analysis(did, grading_func) <<
    ", median(didnt) = " << analysis(didnt, grading_func) << endl;
}

int main() {
  vector<Student> did, didnt;

  Student st;
  while (read(cin, st)) {
    if (did_all_hw(st))
      did.push_back(st);
    else:
      didnt.push_back(st);
  }

  if (did.empty()) {
    cout << "No student did the homework!" << endl;
    return 1;
  }

  if (didnt.empty()){
    coutn << "Every student did the homework!" << endl;
    return 1;
  }

  write_analysis(cout, "median", median_grade, did, didnt);
  write_analysis(cout, "average", average_grade, did, didnt);
  write_analysis(cout, "optimistic", optimistic_median_grade, did, didnt);

  return 0;
}
