#include <vector>
#include <iostream>
#include <string>
#include <cctype>
#include <algorithm>

using namespace std;

struct Student {
  string name;
  double midterm, final;
  vector<double> homework;
};

double median(const vector<double>& vec) {
  vector<double> vecC = vec;
  sort(vecC.begin(), vecC.end());

  vector<double>::size_type width = vecC.size();
  vector<double>::size_type mid = width / 2;
  return width % 2 == 0 ? (vecC[mid]+vecC[mid+1])/2 : vecC[mid];
}

double grade(const Student& st) {
  return st.midterm*0.3 + 0.3*st.final + 0.4*median(st.homework);
}

bool fail(const Student& st) {
  return grade(st) < 60;
}

istream& read(istream& in, vector<Student> vec) {
  in.clear();
  vec.clear();

  while (in) {
    Student st;
    in >> st.name >> st.midterm >> st.final;

    double sc;
    while (in >> sc) {
      st.homework.push_back(sc);
    }
  }

  return in;
}

int main() {
  vector<Student> vecS;
  read(cin, vecS);


  return 0;
}
