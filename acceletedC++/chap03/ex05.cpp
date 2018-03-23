#include <vector>
#include <algorithm>
#include <iostream>
#include <string>

using namespace std;

int main() {
  vector<string> names;
  vector<double> grades;
  int numOfGrades = 5;

  while (1) {
    cout << "Please enter your first name: ";
    string name;
    cin >> name;
    if (name == "end") break;
    cout << "Hello, " << name << "!" << endl;

    cout << "Please enter your midterm and final exam grades: ";
    double midterm, final;
    cin >> midterm >> final;

    cout << "Enter all your homework grades, followed by end-of-file: ";

    double x;
    vector<double> homework;

    for (int count = 0; count < numOfGrades; count++) {
      cin >> x;
      homework.push_back(x);
    }

    typedef vector<double>::size_type vec_sz;
    vec_sz size = homework.size();

    if (size == 0) {
      cout << endl << "You must enter your grades for homework."
        "Please try again." << endl;
      return 1;
    }

    sort(homework.begin(), homework.end());
    vec_sz mid = size / 2;
    double median;
    median = size % 2 == 0 ? (homework[mid] + homework[mid-1]) / 2 : homework[mid];

    double grade = 0.2 * midterm + 0.4 * final + 0.4 * median;

    grades.push_back(grade);
    names.push_back(name);
  }

  cout << endl << "Studens: " << endl;
  for (int i = 0; i < grades.size(); i++) {
    cout << names[i] << " " << grades[i] << endl;
  }

  return 0;
}
