// #include <array>
#include <iostream>
#include <iterator>

struct Rectangle {
  int length, width;
};

void passArray(int prime[5]) {
  prime[0] = 999;
}

void printSize(int array[]) {
  std::cout << "Inside function array size: " << sizeof(array) << "\n";
}

int main() {
  double array[3] = {2, 3, 4};
  Rectangle rets[2] = {{1, 3}, {4, 5}};
  std::cout << array[0] / 3 << " " << rets[0].length << "\n";

  //std::array<int, 5> runtimeArr {2, 3, 4, 5, 12};
  //std::cout << runtimeArr[7];

  const int length = 6;
  int runtimeArr_[length] {1, 2, 3, 8, 9};

  std::cout << "Before pass " << runtimeArr_[0] << "\n";
  passArray(runtimeArr_);
  std::cout << "After pass " << runtimeArr_[0] << "\n";

  std::cout << "Inside main array size: " <<  sizeof(runtimeArr_) << "\n";
  printSize(runtimeArr_);

  std::cout << "indirect way get array length: " << sizeof(runtimeArr_)/sizeof(runtimeArr_[0]) << "\n";
  std::cout << "direct way get array length: " << std::size(runtimeArr_) << "\n";

  int scores[]  { 84, 62, 76, 89, 56 };
  const int numStudents = std::size(scores);

  double totalScore = 0;

  for (auto e : scores)
    totalScore += e;

  std::cout << "Average score is " << totalScore / numStudents << "\n";

  std::cout << "==========================\n";
  for (auto e : scores)
    std::cout << e << " ";
  std::cout << "\n";

  std::cout << "==========================\n";
  int index = -1;
  do {
    std::cout << "Please enter a number between 0 and " << std::size(scores) << "(not included): ";
    std::cin >> index;
    if (std::cin.fail()) std::cin.clear();
    std::cin.ignore(32767, '\n');
  } while ( (index < 0) || (index >= std::size(scores)) );

  std::cout << "what you select is: " << scores[index] << "\n";

  std::cout << "==========================\n";
  int maxScoreIndex = -1;
  for (int i = 0; i < numStudents; i++) {
    if (maxScoreIndex < 0)
      maxScoreIndex = i;
    else if (scores[maxScoreIndex] < scores[i])
      maxScoreIndex = i;
  }
  if (maxScoreIndex >= 0)
    std::cout << "The best score was " << scores[maxScoreIndex] << " at index " << maxScoreIndex << "\n";
}
