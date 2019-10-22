#pragma once

class Date {
  private:
    int year, month, day;

  public:
    Date(int year_, int month_, int day_);
    void setDate(int year_, int month_, int day_);
    int getYear() { return year; };
    int getMonth() { return month; };
    int getDay() { return day; };
};


