#include <iomanip>
#include <iostream>
#include <string>

using std::cout;
using std::cin;
using std::endl;
using std::istream;
using std::ostream;
using std::streamsize;
using std::setw;
using std::to_string;
using std::setprecision;

int main() {
  double maxlen = 21;
  streamsize maxw = to_string(maxlen*maxlen).size();
  streamsize minw = to_string(maxlen).size();
  streamsize defaultlen = cout.width();
  for (int i = 0; i<=maxlen; ++i) {
    int square = i * i;
    cout << setw(minw) << i << " " << setw(maxw) << square << endl;
  }
  cout.width(defaultlen);
  return 0;
}
