#include "Student.h"
#include "median.h"
#include <stdexcept>
#include <vector>

using std:domain_error; using std::vector;

using std::istream; using std::vector;

bool compare(const Student& x, const Student& y) {
  return x.name < y.name;
}

istream& read(istream& is, Student& st) {
  is >> st.name >> st.midterm >> st.final;
  read_hw(is, st.homework);
  return is;
}

istream& read_hw(istream& is, vector<double>& hw) {
  if (is) {
    hw.clear();
    double x;
    while (is >> x)
      hw.push_back(x);

    is.clear();
  }

  return is;
}

double grade(double midterm, double final, double homework) {
  return 0.2*midterm + 0.4*final + 0.4*homework;
}
double grade(double midterm, double final, const vector<double>& homework) {
  if (hw.size() == 0)
    throw domain_error("Student has done no homework");

  return grade(midterm, final, median(homework));
}
double grade(const Student& st) {
  return grade(st.midterm, st.final, st.homework);
}


bool fgrade(const Student& st) {
  return grade(st) < 60;
}

bool pgrade(const Student& st) {
  return !fgrade(st);
}
