#include <opencv2/opencv.hpp>
#include <iostream>

int main() {
  cv::Mat i1 = cv::imread("./test.png", CV_LOAD_IMAGE_GRAYSCALE);
  cv::Mat_<uint8_t> i2 = cv::imread("./test.png", CV_LOAD_IMAGE_GRAYSCALE);
  std::cout << i1.type() << " " << i2.type() << " " << (i1.type() == i2.type()) << std::endl;
  std::cout << i1.cols << std::endl;

  cv::imwrite("copy.jpg", i1);
  return 0;
}
