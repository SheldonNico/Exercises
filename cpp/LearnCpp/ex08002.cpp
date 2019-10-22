#include <iostream>

class Date {
  private:
    int year, month, day;
  public:
    void setDate(int year_, int month_, int day_) {
      year = year_; month = month_; day = day_;
    }
    void print() {
      std::cout << year << "/" << month << "/" << day << "\n";
    }
};

int main() {
  Date dt;
  dt.setDate(2019, 05, 17);
  dt.print();



  return 0;
}
