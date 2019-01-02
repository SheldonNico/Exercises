#include <string>
#include <vector>

class Student {
private:
  std::string nm;
  double midterm, final;
  std::vector<double> homework;

public:
  Student ();
  Student (std::istream&);

  std::istream& read(std::istream&);
  double grade() const;
  std::string name() const { return nm; }
  bool valid() const { return !homework.empty(); }
};

Student::Student(): midterm(0), final(0) {};
Student::Student(istream& is) { read(is); };

std::istream& Student::read(std::istream& in) {
  in >> nm >> midterm >> final;
  read_hw(in, homework);
  return in;
}

std::istream& read_hw(std::istream& in, std::vector<double>& hw) {
  while (in) {
    double score;
    in >> score;
    hw.push_back(score);
  }
  in.clear();
  return in;
}

double Student::grade() const {
  return ::grade(midterm, final, homework);
}

bool compare(const Student& x, const Student& y) {
  return x.name() < y.name();
}

int main() {
  std::vector<Student> students;
  Student record;
  string::size_type maxlen = 0;

  while (record.read(std::cin)) {
    maxlen = max(maxlen, record.name().size());
    students.push_back(record);
  }

  sort(students.begin(), students.end(), compare);

  for (std::vector<Student>::size_type i = 0;
       i != students.size();
       ++i ) {
    std::cout << students[i].name()
              << string(maxen+1-students[i].name().size(), ' ');
    try {
      double final_grade = students[i].grade();
      streamsize prec = cout.precsion();
      cout << setprecision(3) << final_grade << setprecision(prec) << endl;
    } catch (domain_error e) {
      cout << e.what() << endl;
    }
  }
}
