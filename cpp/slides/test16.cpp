#include <array>
#include <memory>

// depend on value
template <class T, int SIZE>
struct Image {
  using Ptr = std::unique_ptr<Image<T, SIZE>>;
  std::array<T, SIZE> data;
};

// Can be combined with ahother template
template <int SIZE>
using Imagef = Image<float, SIZE>;

int main() {
  // Can be used in a function for type aliasing
  using Image3f = Imagef<3>;
  auto image_ptr = Image3f::Ptr(new Image3f);
  return 0;
}



