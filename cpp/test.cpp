#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
      if (s.size() <= 1) return s.size();

      int maxLen {0}, first {0};
      std::map<char, int> lastSeenMap;

      for (int i {0}; i < s.size(); i++) {
        maxLen = i - first > maxLen ? i - first : maxLen;
        if (lastSeenMap.find(s[i]) != lastSeenMap.end() && lastSeenMap[s[i]] >= 0) {
          first = lastSeenMap[s[i]];
          for (auto& v : lastSeenMap) {
            if (v.second < first) {
              lastSeenMap[s[i]] = -1;
            }
          }
          lastSeenMap[s[i]] = i;
          first = i;
        } else {
          lastSeenMap[s[i]] = i;
        }
        std::cout << i << ": " << s[i] << first << "\n";
      }

      if (s[first] != s.back()) {
        maxLen = s.size() - first > maxLen ? s.size() - first : maxLen;
      }

      return maxLen;
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

int main() {
    string line;
    std::cout << Solution().lengthOfLongestSubstring("dvdf") << "\n";
    return 0;
}
