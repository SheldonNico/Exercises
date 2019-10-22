#include <iostream>
#include <cassert>

class Point {
  private:
    double m_x, m_y, m_z;

  public:
    Point(double x = 0.0, double y = 0.0, double z = 0.0) : m_x{x}, m_y{y}, m_z{z} {};
    friend std::ostream& operator<<(std::ostream& buf, const Point& point);

    double& operator[] (const int index);
    const double& operator[] (const int index) const;
};

std::ostream& operator<<(std::ostream& buf, const Point& point) {
  buf << "Point(" << point.m_x << ", " << point.m_y << ", " << point.m_z << ")";
  return buf;
}

double& Point::operator[] (const int index) {
  assert(index >= 0 && index < 3 && "only 3 element to select");
  switch (index) {
    case(0) : return m_x;
    case(1) : return m_y;
    case(2) : return m_z;
    default : return m_x;
  }
}

const double& Point::operator[] (const int index) const {
  assert(index >= 0 && index < 3 && "only 3 element to select");
  switch (index) {
    case(0) : return m_x;
    case(1) : return m_y;
    case(2) : return m_z;
    default : return m_x;
  }
}

int main() {
  std::cout << Point {2, 3, 4} << "\n";

  Point p {1, 2, 3};
  p[0] = 12;
  std::cout << p[0] << "\n";

  return 0;
}
