// generic functions
#include <stdexcept>
#include <vector>
#include <iostream>
#include <iterator>
#include <ctype.h>
#include <algorithm>

using namespace std;

template <class T> T median(vector<T> v) {
  typedef typename vector<T>::size_type vec_sz;

  vec_sz size = v.size();
  if (size == 0)
    throw domain_error("median of an empty vector");

  sort(v.begin(), v.end());
  vec_sz mid = size / 2;

  return size % 2 == 0 ? (v[mid] + v[mid-1])/2 : v[mid];
}

template <class In, class X> In findA(In begin, In end, const X& x) {
  if (begin == end || *begin == x)
    return begin;
  begin++;

  return find(begin, end, x);
}

template <class In, class X> In findB(In begin, In end, const X& x) {
  while (begin != end && *begin != x)
    begin++;
  return begin;
}

template <class In, class Out> Out copyA(In begin, In end, Out dest) {
  while (begin != end)
    *dest++ = *begin++;
  return dest;
}

template <class For, class X> void replace(For beg, For end, const X& x, const X& y) {
  while (beg != end) {
    if (*beg == x)
      *beg = y;
    ++beg;
  }
}

template <class Bi> void reverse(Bi begin, Bi end) {
  while (begin != end){
    --begin;
    if (begin != end)
      swap(*begin++, *end);
  }
}

bool is_space(const char& c) {
  return isspace(c);
}

bool not_space(const char& c) {
  return !is_space(c);
}

template <class Out> void split(const string& str, Out os) {
  typedef string::const_iterator iter;

  iter i = str.begin();
  while (i != str.end()) {
    i = find(i, str.end(), not_space);

    iter j = find(i, str.end(), is_space);
    if (i != str.end())
      *os++ = string(i, j);

    i = j;
  }
}


template <class Ran, class X> bool binary_search(Ran begin, Ran end, const X& x) {
  while (begin < end) {
    Ran mid = (begin + end) / 2;
    if (x < *mid)
      end = mid;
    else if (x > *mid)
      begin = mid;
    else
      return true;
  }

  return false;
}


int main() {
  vector<int> arr = { 1, 2, 3, 4 };
  cout << *findA(arr.begin(), arr.end(), 4) << endl;
  cout << *findB(arr.begin(), arr.end(), 4) << endl;

  vector<int> v;
  copy(istream_iterator<int>(cin), istream_iterator<int>(), back_inserter(v));
  copy(v.begin(), v.end(), ostream_iterator<int>(cout, " "));
  cout << endl;
  return 0;
}
