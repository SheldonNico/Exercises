#include <iostream>
#include <stdexcept>
#include <vector>
#include <list>
#include <cctype>

using std::cin;
using std::cout;
using std::endl;
using std::domain_error;
using std::string;
using std::vector;
using std::list;
using std::max;

struct Student {
  double midterm, final;
  vector<double> homework;
};

bool fgrade(Student st) {
  return (st.midterm < 10);
}

list<Student> extract_fails(list<Student>& students) {
  list<Student> fail;
  list<Student>::iterator iter = students.begin();

  while (iter != students.end()) {
    if (fgrade(*iter)) {
      fail.push_back(*iter);
      iter = students.erase(iter);
    } else {
      ++iter;
    }
  }
  return fail;
}

vector<string> split(const string& s) {
  vector<string> ret;
  string::size_type i = 0;

  while (i != s.size()) {
    while (i != s.size() && isspace(s[i]))
      ++i;

    string::size_type j = i;
    while (j != s.size() && !isspace(s[j]))
      ++j;

    if (i != j) {
      ret.push_back(s.substr(i, j-i));
      i = j;
    }
  }

  return ret;
}

string::size_type width(const vector<string>& v) {
  string::size_type maxlen = 0;
  for (vector<string>::size_type i=0; i != v.size(); ++i)
    maxlen = max(maxlen, v[i].size());
  return maxlen;
}

vector<string> frame(const vector<string>& v) {
  vector<string> ret;
  string::size_type maxlen = width(v);
  string border(maxlen + 4, '*');

  ret.push_back(border);
  for (vector<string>::size_type i=0; i != v.size(); ++i)
    ret.push_back("* " + v[i] + string(maxlen-v[i].size(), ' ') + " *");

  ret.push_back(border);

  return ret;
}

vector<string> vcat(const vector<string>& top, const vector<string>& bottom) {
  // assignment will make a copy
  vector<string> ret = top;
  // for (vector<string>::const_iterator it=bottom.begin(); it != bottom.end(); ++it)
  //   ret.push_back(*it);
  ret.insert(ret.end(), bottom.begin(), bottom.end());
  return ret;
}

vector<string> hcat(const vector<string>& left, const vector<string>& right) {
  vector<string> ret;

  vector<string>::size_type i = 0, j = 0;
  string::size_type widthl = width(left);

  while (i != left.size() || j != right.size()) {
    string s;
    if (i != left.size())
      s += left[i++];
    s += string(widthl-s.size(), ' ');

    if (j != right.size())
      s += right[j++];

    ret.push_back(s);
  }
  return ret;
}

int main () {
  cout << "Hello world!" << endl;
  return 0;
}
