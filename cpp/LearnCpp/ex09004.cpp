#include <iostream>
#include <string>
#include <vector>

struct Student {
  std::string m_name;
  char m_grade;
};

class GradeMap {
  private:
    std::vector<Student> m_map;

  public:
    GradeMap() : m_map{} {};
    char& operator[] (const std::string& index) {
      for (auto& s : m_map) {
        if (s.m_name == index)
          return s.m_grade;
      }

      m_map.push_back({ index, 0 });
      return m_map.back().m_grade;
    }
};

int main() {
  GradeMap grades;
  //grades["Joe"] = 'A';
  //grades["Frank"] = 'B';
  //std::cout << "Joe has a grade of " << grades["Joe"] << '\n';
  //std::cout << "Frank has a grade of " << grades["Frank"] << '\n';

  char& gradeJoe = grades["Joe"]; // does a push_back
  gradeJoe = 'A';

  char& gradeFrank = grades["Frank"]; // does a push_back
  gradeFrank = 'B';

  std::cout << "Joe has a grade of " << gradeJoe << '\n';
  std::cout << "Frank has a grade of " << gradeFrank << '\n';

  return 0;
}
