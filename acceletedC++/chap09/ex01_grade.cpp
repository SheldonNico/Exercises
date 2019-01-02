#include <vector>
#include <stdexcept>
#include "ex01_grade.hpp"

using std::vector;

double grade(double midterm, double final, double homework) {
  return 0.2*midterm + 0.4*final + 0.4*homework;
}

double grade(double midterm, double final, const vector<double>& homework) {
  if (homework.size() == 0)
    throw domain_error("Student has done no homwork!");

  return grade(midterm, final, median(homework));
}
