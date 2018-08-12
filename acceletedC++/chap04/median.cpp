#include <algorithm>
#include <stdexcept>
#include <vector>

using std::domain_error;
using std::sort;
using std::vector;

double median(vector<double> vs) {
  if (vs.size() == 0) {
    throw domain_error("length not right.");
  } else {
    vector<double>::size_type mid = vs.size() / 2;
    return vs.size() % 2 == 0 ? (vs[mid] + vs[mid-1])/2 : vs[mid];
  }
}
