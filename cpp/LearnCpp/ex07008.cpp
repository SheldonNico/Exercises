#include <iostream>

int binarySearch(int* array, int target, int min, int max) {
  int middle = 0;
  while (min <= max) {
    middle = (min + max) / 2;
    if (target == array[middle]) {
      return middle;
    } else if (target > array[middle]) {
      min = middle + 1;
    } else {
      max = middle - 1;
    }
  }
  return -1;
}

int binarySearch2(int* array, int target, int min, int max) {
  int middle = 0;

  if (min <= max) {
    middle = ( min+max ) / 2;
    if (target == array[middle]) {
      return middle;
    } else if (target > array[middle]) {
      return binarySearch2(array, target, middle+1, max);
    } else {
      return binarySearch2(array, target, min, middle-1);
    }
  } else {
    return -1;
  }

}

double max(const double& num1, const double& num2) {
  return num1 > num2 ?  num1 : num2;
}

void swap(int& num1, int& num2) {
  num1 = num1 + num2;
  num2 = num1 - num2;
  num1 = num1 - num2;
}

int& getLargestElement(int* array, const int& len) {
  int max = 0;
  for (int count {0}; count < len; count++ ) {
    if (array[count] > array[max]) {
      max = count;
    }
  }

  return array[max];
}


int main() {
  int array[] = { 3, 6, 8, 12, 14, 17, 20, 21, 26, 32, 36, 37, 42, 44, 48 };

  // We're going to test a bunch of values to see if they produce the expected results
  const int numTestValues = 9;
  // Here are the test values
  int testValues[numTestValues] = { 0, 3, 12, 13, 22, 26, 43, 44, 49 };
  // And here are the expected results for each value
  int expectedValues[numTestValues] = { -1, 0, 3, -1, -1, 8, -1, 13, -1 };

  // Loop through all of the test values
  for (int count=0; count < numTestValues; ++count)
  {
    // See if our test value is in the array
    int index = binarySearch2(array, testValues[count], 0, 14);
    // If it matches our expected value, then great!
    if (index == expectedValues[count])
      std::cout << "test value " << testValues[count] << " passed!\n";
    else // otherwise, our binarySearch() function must be broken
      std::cout << "test value " << testValues[count] << " failed.  There's something wrong with your code!\n";
  }


  return 0;
}
