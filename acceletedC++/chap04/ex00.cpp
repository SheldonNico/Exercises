#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <iomanip>
#include <stdexcept>
#include "grade.h"
#include "student.h"

using std::cin;
using std::cout;
using std::max;
using std::domain_error;
using std::endl;
using std::streamsize;
using std::setprecision;
using std::vector;
using std::string;

int main () {
  vector<Student> students;
  Student record;
  string::size_type maxlen = 0;

  while (read(cin, record)) {
    maxlen = max(record.name.size(), maxlen);
    students.push_back(record);
  }

  sort(students.begin(), students.end(), compare);

  for (vector<Student>::size_type i = 0; i != students.size(); ++i) {
    cout << students[i].name << string(maxlen+1-students[i].name.size(), ' ');
    try {
      double final = grade(students[i]);
      streamsize prec = cout.precision();

      cout << setprecision(3) << final << setprecision(prec);
    } catch (domain_error& e) {
      cout << e.what();
    }

    cout << endl;
  }
  return 0;
}
