#include <iostream>
#include <string>
#include <vector>
#include <tuple>
#include <list>
#include <cctype>

using namespace std;

vector<string> split(const string& str) {
  vector<string> out;

  int i = 0;
  if (i != str.size()) {
    while (isspace(str[i]))
      ++i;
  }

  while (i != str.size()) {

    int j = i;
    while (!isspace(str[j]) && j != str.size())
      j++;

    out.push_back(str.substr(i, j-i));

    i = j;
    while (isspace(str[i]))
      ++i;
  }

  return out;
}

string concat(const vector<string> vecs) {
  string ret;
  for (vector<string>::const_iterator it=vecs.begin(); it != vecs.end(); ++it) {
    if (ret.size() == 0)
      ret = (*it);
    else
      ret += " " + (*it);
  }
  return ret;
}

bool cmp(const tuple<string, string>& sa, const tuple <string, string>& sb) {
  string ca = get<1>(sa), cb = get<1>(sb);
  int i = 0;
  while (i != ca.size() && i != cb.size()) {
    int cai = tolower(ca[i]), cbi = tolower(cb[i]);
    if (cai == cbi)
      i++;
    else
      return cai < cbi;

  }
  return ca.size() < cb.size();
}

list<tuple<string, string>> kic(const list<string>& lines) {
  list<tuple<string, string>> out;
  for (list<string>::const_iterator it=lines.begin(); it != lines.end(); ++it) {
    vector<string> line = split(*it);
    for (int i=0; i != line.size(); ++i) {
      vector<string> vh(&line[0], &line[i]);
      vector<string> ve(&line[i], &line[line.size()]);
      string strh = concat(vh);
      string stre = concat(ve);
      out.push_back(make_tuple(strh, stre));
    }
  }
  out.sort(cmp);
  return out;
}

ostream& display(ostream& out, const list<tuple<string, string>>& li) {
  string::size_type width = 0;
  for (list<tuple<string, string>>::const_iterator it=li.begin(); it != li.end(); ++it) {
    width = max(width, (get<0>(*it)).size());
  }
  for (list<tuple<string, string>>::const_iterator it=li.begin(); it != li.end(); ++it) {
    string stringa=get<0>(*it), stringb=get<1>(*it);
    out << string(width-stringa.size(), ' ') << stringa << "    " << stringb << endl;
  }

  return out;
}

int main() {

  list<string> content;
  content.push_back("The quick brown fox");
  content.push_back("jumped over the fences");

  list<tuple<string, string>> formatLine = kic(content);
  display(cout, formatLine);
  return 0;
}
