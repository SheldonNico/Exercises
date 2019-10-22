#include <iostream>
#include <vector>

class Point {
  private:
    int m_x, m_y, m_z;

  public:
    Point(int x, int y, int z) : m_x {x}, m_y {y}, m_z {z} {}

    friend std::ostream& operator<<(std::ostream& os, const Point& p) {
      os << "Point(" << p.m_x << ", " << p.m_y << ", " << p.m_z << ")";
      return os;
    }
};

class Shape {
  public:
    virtual std::ostream& print(std::ostream& os) const = 0;
    friend std::ostream& operator<<(std::ostream& os, const Shape& s) {
      return s.print(os);
    }
    virtual ~Shape() {};
};

class Circle : public Shape {
  private:
    Point m_c;
    int m_r;

  public:
    Circle(const Point& p, int r) : m_c {p}, m_r {r} {}
    virtual std::ostream& print(std::ostream& os) const override {
      os << "Circle(" << m_c << ", radius " << m_r << ")";
      return os;
    }
    const int& getRadius() const { return m_r; }
    virtual ~Circle() override {};
};

class Triangle : public Shape {
  private:
    Point m_x, m_y, m_z;

  public:
    Triangle(const Point& x, const Point& y, const Point& z) :
      m_x {x}, m_y {y}, m_z {z} {}

    virtual std::ostream& print(std::ostream& os) const override {
      os << "Triangle(" << m_x << ", " << m_y << ", " << m_z << ")";
      return os;
    }
    virtual ~Triangle() override {};
};

int getLargestRadius(const std::vector<Shape*> vec) {
  int r {0};
  for (const auto& s : vec) {
    Circle* c = dynamic_cast<Circle*>(s);
    if (c && (c->getRadius() > r)) {
      r = c->getRadius();
    }
  }
  return r;
}


int main() {
  Circle c(Point(1, 2, 3), 7);
  std::cout << c << '\n';

  Triangle t(Point(1, 2, 3), Point(4, 5, 6), Point(7, 8, 9));
  std::cout << t << '\n';

  std::cout << "\nassignaments:\n";

  std::vector<Shape*> v;
  v.push_back(new Circle(Point(1, 2, 3), 7));
  v.push_back(new Triangle(Point(1, 2, 3), Point(4, 5, 6), Point(7, 8, 9)));
  v.push_back(new Circle(Point(4, 5, 6), 3));

  // print each shape in vector v on its own line here
  for (const auto& s : v) {
    std::cout << *s << "\n";
  }

  std::cout << "The largest radius is: " << getLargestRadius(v) << '\n'; // write this function


  // delete each element in the vector here
  for (auto& s : v) {
    delete s;
  }
  return 0;
}
