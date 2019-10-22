#pragma once

#include <vector>
#include <stdexcept>
#include <string>
#include "io_tools.h"

namespace igg {

  class Image {
  public:
    Image(int rows, int cols);
    Image();
    const int& rows() const { return rows_; }
    const int& cols() const { return cols_; }
    const int& at(const int& row, const int& col) const;
    bool FillFromPgm(const std::string& filename);

    void WriteToPgm(const std::string filename);

    std::vector<float> ComputeHistogram(const int& bins) const;

    void DownScale(const int& scale);
    void UpScale(const int& scale);

  private:
    int rows_ = 0;
    int cols_ = 0;
    int max_val_ = 255;
    std::vector<int> data_;
  };

}  // namespace igg
