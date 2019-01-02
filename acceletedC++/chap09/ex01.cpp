#include <vector>
#include <string>
#include <algorithm>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include "ex01_Student.cpp"

using namespace std;

int main() {
  vector<Student> students;
  Student record;
  string::size_type maxlen = 0;

  while (record.read(cin)) {
    maxlen = max(maxlen, record.name().size());
    students.push_back(record);
  }

  sort(students.begin(), students.end(), compare);

  for (vector<Student>::size_type i = 0; i != students.size(); ++i) {
    Student st = students[i];
    cout << st.name() << string(maxlen+1-st.name().size(), ' ');

    try {
      double final_grade = st.grade();
      streamsize prec = cout.precision();
      cout << setprecision(3) << final_grade << setprecision(prec) << endl;
    } catch (domain_error e) {
      cout << e.what() << endl;
    }
  }

  return 0;
}
