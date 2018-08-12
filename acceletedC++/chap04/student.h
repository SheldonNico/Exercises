#ifndef GUARD_student
#define GUARD_student
#include <iostream>
#include <string>
#include <vector>

struct Student {
  std::string name;
  double midterm, final;
  std::vector<double> homework;
};

bool compare(const Student&, const Student&);
std::istream& read(std::istream&, Student&);
std::istream& read_hw(std::istream&, std::vector<double>&);
#endif
