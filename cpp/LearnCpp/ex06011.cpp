#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

struct Student {
  std::string name;
  double grade;
};

int main() {
  std::cout << "how many student will you enter: ";
  int max_len;
  std::cin >> max_len;

  Student* students = new Student[max_len];
  for (int count {0}; count < max_len; ++count) {
    std::cout << "Enter name and grade for #" << count << ": ";
    std::cin >> students[count].name;
    std::cin >> students[count].grade;
  }

  std::sort(students, students+max_len, [](Student s1, Student s2) {return s1.grade > s2.grade; } );

  for (int count {0}; count < max_len; ++count) {
    std::cout << "student #" << count << ": " << students[count].name << " " << students[count].grade << "\n";
  }


  return 0;
}
