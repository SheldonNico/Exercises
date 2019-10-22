#include <iostream>
#include <string>
#include <cmath>

class Point2d {
  private:
    double m_x, m_y;

  public:
    Point2d(double x=0, double y=0) : m_x{x}, m_y{y} {};

    std::string show() const {
      return std::string("Point2d(") + std::to_string(m_x) + std::string(", ") + std::to_string(m_y) + std::string(")");
    }

    double distanceTo(Point2d const& p2) const {
      return std::sqrt( std::pow(m_x - p2.m_x, 2) + std::pow(m_y - p2.m_y, 2) );
    }

    friend double distanceFrom(Point2d const& p1, Point2d const& p2);
};


double distanceFrom(Point2d const& p1, Point2d const& p2) {
      return std::sqrt( std::pow(p1.m_x - p2.m_x, 2) + std::pow(p1.m_y - p2.m_y, 2) );
}

int main() {
  Point2d first;
  Point2d second(3.0, 4.0);

  std::cout << first.show() << "\n";
  std::cout << second.show() << "\n";
  std::cout << "Distance between two points: " << first.distanceTo(second) << '\n';
  std::cout << "Distance between two points: " << distanceFrom(first, second) << '\n';
  return 0;
}
