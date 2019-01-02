#include <string>
#include <vector>

class Student {
public:
  Student();
  Student(std::istream&);
  std::string name() const { return nm; };
  bool valid() const { return !homework.empty(); };
  std::istream& read(std::istream&);
  double grade() const { return g; };

private:
  std::string nm;
  double midterm, final, g;
  std::vector<double> homework;
};


std::istream& read_hw(std::istream&, std::vector<double>&);
bool compare(const Student&, const Student&);
