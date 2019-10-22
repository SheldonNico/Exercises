#include <iostream>
#include "ex08007.h"

Date::Date(int year_, int month_, int day_) {
  setDate(year_, month_, day_);
}
void Date::setDate(int year_, int month_, int day_) {
  year = year_; month = month_; day = day_;

}

int main() {
  Date dt { 2019, 5, 9 };

  std::cout << dt.getMonth() << "\n";
  return 0;
}
