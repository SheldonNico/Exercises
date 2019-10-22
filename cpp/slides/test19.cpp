#include <opencv2/opencv.hpp>
#include <iostream>
#include <bits/stdc++.h>

using namespace std;
using namespace cv;

void printVector(vector<int> v) {
  for_each(v.begin(), v.end(), [](int i) {
    std::cout << i << " ";
  });
  std::cout << std::endl;
}

int main() {
  Mat mat = Mat::zeros(10, 10, CV_8UC3);
  std::cout << mat.at<Vec3f>(5, 5) << std::endl;

  vector<int> v {1, 7, 4, 1, 2, 9, 12, 0, 7, 3, 5};
  printVector(v);

  vector<int>::iterator p = find_if(
      v.begin(), v.end(), [](int i) {
      return (i > 5); });
  std::cout << "First number greater than 5 is: " << *p << std::endl;
  int dist = p - v.begin();
  std::cout << "So this is the index: " << dist << std::endl;

  sort(v.begin(), v.end(), [](const int& a, const int& b) -> bool {
      return (a > b);
      });
  printVector(v);

  auto count_greater_than_5 = count_if(v.begin(), v.end(), [](const int& i) -> bool {
      return i > 5;
      });

  std::cout << "There is " << count_greater_than_5 << " numbers more than 5" << std::endl;

  p = unique(v.begin(), v.end(), [](const int& a, const int& b) -> bool {
      return (a == b);
      });
  v.resize(distance(v.begin(), p));
  printVector(v);
  
  int arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  int f = accumulate(arr, arr+10, 1, [](const int& x, const int& acc) -> int {
      return x * acc;
      });
  std::cout << "Fractorial of 10 is: " << f << std::endl;

  auto square = [](int i) -> float {
    return (i*i*1.0);
  };

  std::cout << "lambda use as value: square(5) is " << square(5) << std::endl;

  return 0;
}
