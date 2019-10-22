#include <bits/stdc++.h>

using namespace std;

class Solution {
public:

    string convert(string s, int numRows) {
      std::string out;
      if (numRows == 1) return s;

      std::vector<string> matrix( min(numRows, int(s.size())) );
      bool goingD = true;
      int currentPos {0};

      for (auto si : s) {
        matrix[currentPos] += si;
        if ( currentPos == 0 || (currentPos == numRows-1) ) {
          goingD = !goingD;
        }

        currentPos += goingD ? 1 : -1;
      }

      for (auto si : matrix) {
        out += si;
      }

      return out;
    }
};

string stringToString(string input) {
    assert(input.length() >= 2);
    string result;
    for (int i = 1; i < input.length() -1; i++) {
        char currentChar = input[i];
        if (input[i] == '\\') {
            char nextChar = input[i+1];
            switch (nextChar) {
                case '\"': result.push_back('\"'); break;
                case '/' : result.push_back('/'); break;
                case '\\': result.push_back('\\'); break;
                case 'b' : result.push_back('\b'); break;
                case 'f' : result.push_back('\f'); break;
                case 'r' : result.push_back('\r'); break;
                case 'n' : result.push_back('\n'); break;
                case 't' : result.push_back('\t'); break;
                default: break;
            }
            i++;
        } else {
          result.push_back(currentChar);
        }
    }
    return result;
}

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
  std::cout << Solution().convert("abc", 2);
  //std::cout << "================\n";
  //std::cout << Solution().convert("ABCDEFGHIJKLMNOPQRST", 5);
  //std::cout << "================\n";
  //std::cout << Solution().convert("PAYPALISHIRING", 4);
}
