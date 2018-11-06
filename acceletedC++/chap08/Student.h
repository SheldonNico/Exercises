#ifndef GUARD_Student_h
#define GUARD_Student_h

#include <iostream>
#include <string>
#include <vector>

struct Student {
  std::string name;
  double midterm, final;
  std::vector<double> homework;
};

double grade(double, double, double);
double grade(double, double, const std::vector<double>&);
double grade(const Student&);

bool pgrade(const Student&);
bool fgrade(const Student&);


std::istream& read(std::istream&, Student&);
std::istream& read_hw(std::istream&, std::vector<double>&);

bool compare(const Student&, const Student&);
#endif
