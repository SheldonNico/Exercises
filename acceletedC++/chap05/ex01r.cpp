#include <cctype>
#include <string>
#include <vector>
#include <iostream>
#include <algorithm>
#include <iomanip>

using namespace std;

struct Rotation {
  vector<string>::size_type first;
  vector<string> words;
};

vector<string> split(const string& s) {
  vector<string> ret;
  typedef string::size_type string_size;
  string_size i = 0;

  while (i != s.size()) {
    while (i != s.size() && isspace(s[i]))
      i++;

    string_size j = i;
    while (j != s.size() && !isspace(s[j]))
      j++;

    if (j != i) {
      ret.push_back(s.substr(i, j-i));
      i = j;
    }
  }

  return ret;
}

vector<string> read_lines(istream& in) {
  vector<string> lines;
  string line;

  while (getline(in, line))
    lines.push_back(line);
  return lines;
}

vector<Rotation> rotate_line(const string& line) {
  vector<Rotation> ret;
  vector<string> words = split(line);

  vector<string>::size_type i = 0;
  for (vector<string>::size_type i=0; i < words.size(); ++i) {
    Rotation rot = {words.size()-i, words};
    ret.push_back(rot);
    rotate(words.begin(), words.begin()+1, words.end());
  }

  return ret;
}

vector<Rotation> rotate_lines(vector<string> lines) {
  vector<Rotation> ret;

  for (vector<string>::size_type i=0; i!=lines.size(); ++i) {
    vector<Rotation> out=rotate_line(lines[i]);
    ret.insert(ret.end(), out.begin(), out.end());
  }
  return ret;
}

bool compare(const Rotation& x, const Rotation& y) {
  return x.words < y.words;
}

void print_rotations(vector<Rotation> rotations) {
  vector<string> left, right;
  string::size_type max_left_width = 0;
  for (vector<Rotation>::size_type i=0; i!=rotations.size(); ++i){
    Rotation rot = rotations[i];
    string left_str="", right_str="";
    for (vector<string>::size_type i=rot.first; i!=rot.words.size(); ++i) {
      left_str += " " + rot.words[i];
    }
    left.push_back(left_str);

    if (left_str.size() > max_left_width)
      max_left_width = left_str.size();

    for (vector<string>::size_type i=0; i!=rot.first; ++i)
      right_str += " " + rot.words[i];
    right.push_back(right_str);
  }

  for (vector<string>::size_type i=0; i!=left.size(); ++i) {
    cout << setw(max_left_width);
    cout << left[i];
    cout << "\t";
    cout << right[i];
    cout << endl;
  }
}


int main() {
  vector<string> lines = read_lines(cin);
  vector<Rotation> rots = rotate_lines(lines);
  sort(rots.begin(), rots.end(), compare);
  print_rotations(rots);

  return 0;
}
