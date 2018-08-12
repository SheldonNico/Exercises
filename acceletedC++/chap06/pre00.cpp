#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

struct Student {
  string name;
  double midterm, final;
  vector<double> homework;
};

bool did_all_hw(const Student& s) {
  return (find(s.homework.begin(), s.homework.end(), 0) == s.homework.end());
}

istream& read(istream& in, Student& st) {
  in >> st.name >> st.midterm >> st.final;

  st.homework.clear()
  double v;
  while (in >> v) {
    st.homework.push_back(v);
  }
  in.clear();

  return in;
}

double grade(double mid, double fin, double hw) {
  return 0.4 * mid + 0.4 * fin + 0.2 * hw;
}

double average(const vector<double>& arr) {
  return accumulate(arr.begin(), arr.end(), 0.0) / arr.size();
}

double average_grade(const Student& s) {
  return grade(s.midterm, s.final, average(s.homework));
}

double average_analysis(const vector<Student>& sts) {
  vector<double> grades;
  transform(sts.begin(), sts.end(), back_inserter(grades), average_grade);

  return median(grades);
}

double optimistic_median(const Student& st) {
  vector<double> nonzeros;
  remove_copy(st.homework.begin(), st.homework.end(), back_insert_iterator(nonzeros), 0);

  if (nonzeros.empty())
    return grade(st.midterm, st.final, 0);
  else
    return grade(st.midterm, st.final, median(nonzeros));
}

void write_analysis(ostream& out, const string& name, double analysis(const vector<Student>& sts),
                    const vector<Student>& did, const vector<Student>& didnt) {
  cout << name << ": analysis(did) =" << analysis(did) <<
                  ", analysis(didnt)" << analysis(didnt) << endl;
}

bool pgrade(const Student& st) {
  return grade(st.midterm, st.final, average(st.homework)) > 60;
}

bool fgrade(const Studnet& st) {
  return !pgrade(st);
}

// vector<Student> extract_fails(vector<Student>& sts) {
//   vector<Student> fails;
//   remove_copy_if(sts.begin(), sts.end(), back_inserter(fails), pgrade);
//   sts.erase(remove_if(sts.begin(), sts.end(), fgrade), sts.end());
//   return fails;
// }

vector<Student> extract_fails(vector<Student>& sts) {
  vector<Student>::iterator iter = stable_partition(sts.begin(), sts.end(), pgrade);
  vector<Student> fails(iter, sts.end());
  sts.earse(iter, sts.end());
  return fails;
}

int main() {
  Student student;

  while (read(cin, student)){
    if (did_all_hw(student))
      did.push_back(student);
    else
      didnt.push_back(student);
  }

  if (did.empty()) {
    cout << "No student did all the homework!" << endl;
    return 1;
  }

  if (didnt.empty()) {
    cout << "Every student did all the homework!" << endl;
    return 1;
  }

  write_analysis(cout, "median", median_analysis, did, didnt);
  write_analysis(cout, "average", average_analysis, did, didnt);
  write_analysis(cout, "median of homework turned in", optimistic_median_analysis, did, didnt);

  return 0;
}
