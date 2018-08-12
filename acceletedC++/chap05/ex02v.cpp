#include <vector>
#include <iostream>
#include <string>
#include <cctype>
#include <algorithm>
#include <ctime>

using namespace std;

struct Student {
  string name;
  double midterm, final;
  vector<double> homework;
};

double median(const vector<double>& vec) {
  vector<double> vecC = vec;
  if (vec.size() == 0)
    return 0;

  sort(vecC.begin(), vecC.end());

  vector<double>::size_type width = vecC.size();
  vector<double>::size_type mid = width / 2;
  return width % 2 == 0 ? (vecC[mid]+vecC[mid+1])/2 : vecC[mid];
}

double grade(const Student& st) {
  return st.midterm*0.2 + 0.4*st.final + 0.4*median(st.homework);
}

bool fail(const Student& st) {
  return grade(st) < 60;
}

vector<Student> filter_failed(vector<Student>& sts) {
  vector<Student> failed;
  vector<Student>::iterator it=sts.begin();
  while (it!=sts.end()) {
    if (fail(*it)) {
      failed.push_back(*it);
      it = sts.erase(it);
    } else {
      ++it;
    }
  }
  return failed;
}

istream& read(istream& in, Student& st) {
  in >> st.name >> st.midterm >> st.final;
  if (in) {
    double x;
    while (in >> x) {
      st.homework.push_back(x);
    }

    in.clear();
  }

  return in;
}

int main() {
  vector<Student> vecS, failed;
  Student st;
  while (read(cin, st))
    vecS.push_back(st);

  clock_t start = clock();
  failed = filter_failed(vecS);
  clock_t end = clock();
  cout << "passed students: " << vecS.size() << "\tfailed students: " << failed.size() << endl;
  cout << "Elapsed: " << float(end-start)/CLOCKS_PER_SEC << endl;

  return 0;
}
