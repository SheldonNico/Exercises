#include <iostream>
#include <vector>
#include "ex01_Student.hpp"
#include "ex01_grade.hpp"

using std::istream;
using std::vector;

bool compare(const Student& sa, const Student& sb) {
  return sa.name() < sb.name();
}

Student::Student(): midterm(0), final(0) {}
Student::Student(istream& is) { read(is); }

istream& read_hw(istream& in, vector<double>& homework) {
  if (in) {
    homework.clear();

    double x;
    while (in >> x)
      homework.push_back(x);

    in.clear();
  }
  return in;
}

istream& Student::read(std::istream& in) {
  in >> nm >> midterm >> final;
  read_hw(in, homework);
  g = ::grade(midterm, final, homework);
  return in;
}
