#include <opencv2/opencv.hpp>
#include <iostream>

int main() {
  cv::Mat image = cv::Mat::zeros(0, 0, CV_8UC3);
  using Matf = cv::Mat_<float>;
  Matf image_float = Matf::zeros(10, 10);

  std::cout << image_float << std::endl;
  std::cout << "type: " << image_float.type() << std::endl;
  std::cout << "rows: " << image_float.rows << " cols: " << image_float.cols << std::endl;

  Matf image_no_copy = image_float;
  image_no_copy.at<float>(5, 5) = 42.42f;
  std::cout << image_float.at<float>(5, 5) << std::endl;
  std::cout << image_no_copy.at<float>(5, 5) << std::endl;

  Matf image_copy = image_float.clone();
  image_copy.at<float>(1, 1) = 22;
  std::cout << image_float.at<float>(1, 1) << std::endl;
  std::cout << image_copy.at<float>(1, 1) << std::endl;
  return 0;
}
