#include <string>
#include <cctype>
#include <string>
#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

bool is_space(const char& c) {
  return isspace(c);
}

bool not_space(const char& c) {
  return !isspace(c);
}

bool is_palindrome(const string& s) {
  return equal(s.begin(), s.end(), s.rbegin());
}

vector<string> split(const string& str) {
  vector<string> out;

  typedef string::const_iterator sit;
  sit i = str.begin();
  while (i != str.end()) {
    i = find_if(i, str.end(), not_space);
    sit j = find_if(i, str.end(), is_space);
    if (i != j)
      out.push_back(string(i, j));

    i = j;
  }

  return out;
}

bool is_url_char(const char& c) {
  static const string url_ch = "~;/?:@=&$-_.+!*'(),";
  return (isalnum(c) || find(url_ch.begin(), url_ch.end(), c) != url_ch.end());
}

bool not_url_char(const char& c) {
  return !is_url_char(c);
}

string::const_iterator find_url_end(string::const_iterator& i, string::const_iterator& j) {
  return find_if(i, j, not_url_char);
}

string::const_iterator find_url_beg(string::const_iterator& i, string::const_iterator& j) {
  static const string url_sep = "://";
  string::const_iterator s = i;
  while ((s = search(s, j, url_sep.begin(), url_sep.end())) != j) {
    string::const_iterator beg = s;
    if (s != i && (s + url_sep.size()) != j) {
      while (isalpha(beg[-1]) && beg != i) {
        --beg;
      }

      if (beg != i && is_url_char(s[url_sep.size()]))
        return beg;
    }
    s = beg;
  }

  return j;
}

vector<string> find_url(const string& text) {
  typedef string::const_iterator siter;
  vector<string> out;
  siter i = text.begin(), e = text.end();
  while (i != e) {
    siter url_beg = find_url_beg(i, e);
    siter url_end = find_url_end(url_beg, e);
    if (url_beg != url_end)
      out.push_back(string(url_beg, url_end));
    i = url_end;
  }

  return out;
}

int main() {
  string test = "  this http://some other staff\n and more ftp://122.1212";
  vector<string> out = find_url(test);
  for (vector<string>::const_iterator it=out.begin(); it != out.end(); ++it)
    cout << *it << endl;
  return 0;
}
