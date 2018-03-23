#include <iostream>
#include <string>
#include <ios>
#include <vector>
#include <algorithm>

using std::cout;
using std::cin;
using std::endl;
using std::vector;
using std::sort;

int main() {
  cout << "Please enter the set of number, seperated by space: " << endl;

  vector<double> nums;
  double x;
  while (cin >> x)
    nums.push_back(x);
  sort(nums.begin(), nums.end());

  typedef vector<double>::size_type vec_sz;
  vec_sz size = nums.size();
  if (size == 0) {
    cout << endl << "You must enter at least 1 number."
      "Please try again." << endl;
    return 1;
  }

  // auartile computation is more complex than this
  vec_sz quartle1 = size / 4;
  vec_sz quartle2 = 3 * size / 4;

  cout << "1/4 and 3/4 quartiles is: " <<
    nums[quartle1] << " & " << nums[quartle2] << endl;

  return 0;
}
