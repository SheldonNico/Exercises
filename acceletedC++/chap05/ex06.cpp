#include <vector>
#include <iostream>
#include <string>
#include <cctype>
#include <algorithm>
#include <ctime>
#include <list>

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

// typedef vector<Student> arrStudent;
typedef list<Student> arrStudent;

arrStudent filter_failed(arrStudent& sts) {
  arrStudent failed;
  arrStudent::iterator it=sts.begin();
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

arrStudent extract_failed(arrStudent& sts) {
  arrStudent failed;
  arrStudent::size_type raw_size = sts.size();
  for (arrStudent::iterator it=sts.begin(); it != sts.end(); ++it) {
    if (fail(*it)) {
      failed.push_back(*it);
    } else {
      sts.insert(sts.begin(), *it);
    }
  }
  sts.resize(raw_size - failed.size());
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
  arrStudent lisS, failed;
  Student st;
  while (read(cin, st))
    lisS.push_back(st);

  clock_t start = clock();
  failed = filter_failed(lisS);
  clock_t end = clock();
  cout << "passed students: " << lisS.size() << "\tfailed students: " << failed.size() << endl;
  cout << "Elapsed: " << float(end-start)/CLOCKS_PER_SEC << endl;

  return 0;
}

/*
  no difference actually. causing insert is as slow as erase.
  random access effeciency is the same reason.
*/
