#include <iostream>
#include <vector>

using namespace std;

vector<string> center(const vector<string>& picture) {
  vector<string> new_picture;
  string::size_type maxlen=0;

  for (vector<string>::const_iterator iter=picture.begin(); iter != picture.end(); ++iter)
    maxlen = max(maxlen, iter->size());

  for (vector<string>::const_iterator iter=picture.begin(); iter != picture.end(); ++iter) {
    string::size_type padding = (maxlen - iter->size()) / 2;
    string new_string = string(padding, ' ') + *iter + string(maxlen-padding-iter->size(), ' ');
    new_picture.push_back(new_string);
  }

  return new_picture;
}

int main() {
  vector<string> picture;
  picture.push_back("*");
  picture.push_back("***");
  picture.push_back("*****");

  vector<string> new_pic = center(picture);
  for (vector<string>::const_iterator it=new_pic.begin(); it != new_pic.end(); ++it)
    cout << *it << endl;

  return 0;
}
