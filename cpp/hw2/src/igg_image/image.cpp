#include "image.h"

igg::Image::Image(int rows, int cols) : rows_{rows}, cols_{cols} {};
igg::Image::Image() {};

bool igg::Image::FillFromPgm(const std::string& filename) {
  igg::io_tools::ImageData img = igg::io_tools::ReadFromPgm(filename);
  if (img.data.size() == 0)
    return false;
  else {
    rows_ = img.rows;
    cols_ = img.cols;
    max_val_ = img.max_val;
    data_ = img.data;

    return true;
  }
}

void igg::Image::WriteToPgm(const std::string filename) {
  igg::io_tools::ImageData img{rows_, cols_, max_val_, data_};
  igg::io_tools::WriteToPgm(img, filename);
}

const int& igg::Image::at(const int& row, const int& col) const {
  std::vector<int>::size_type sel = row * cols_ + col;
    if (sel <= data_.size())
      return data_[sel];
    else
      throw std::runtime_error("select outside image!");
}

void igg::Image::DownScale(const int& scale) {
  std::vector<int> newdata;
  int rows = rows_ / scale, cols = cols_ / scale;

  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      newdata[i*rows+cols] = at(i*scale, j*scale);
    }
  }

  rows_ = rows;
  cols_ = cols;
  data_ = newdata;
}

void igg::Image::UpScale(const int& scale) {
  std::vector<int> newdata;
  int rows = rows_ * scale, cols = cols_ * scale;

  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      newdata[i*rows+cols] = at(i/scale, j/scale);
    }
  }

  rows_ = rows;
  cols_ = cols;
  data_ = newdata;
}

std::vector<float> igg::Image::ComputeHistogram(const int& bins) const {
  std::vector<float> hist;
  int index;
  for (auto v : data_) {
    index = v / bins;
    hist[index] += 1;
  }

  return hist;
}
