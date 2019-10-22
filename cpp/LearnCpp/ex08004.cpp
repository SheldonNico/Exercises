#include <iostream>

class Point3d {
  private:
    int x, y, z;

  public:
    void setValues(int x_, int y_, int z_) {
      x = x_; y = y_; z = z_;
    }
    void print() {
      std::cout << "<" << x << ", " << y << ", " << z << ">\n";
    }
    bool isEqual(const Point3d& p) {
      return (x == p.x) && (y == p.y) && (z == p.z);
    }
};

int main() {
  Point3d point;
  point.setValues(1, 2, 3);

  point.print();

  Point3d point1;
  point1.setValues(1, 2, 3);

  Point3d point2;
  point2.setValues(1, 2, 3);

  if (point1.isEqual(point2))
    std::cout << "point1 and point2 are equal\n";
  else
    std::cout << "point1 and point2 are not equal\n";

  Point3d point3;
  point3.setValues(3, 4, 5);

  if (point1.isEqual(point3))
    std::cout << "point1 and point3 are equal\n";
  else
    std::cout << "point1 and point3 are not equal\n";
  return 0;
}
