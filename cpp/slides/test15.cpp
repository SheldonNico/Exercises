#include <iostream>
#include <string>
using namespace std;

int main(int argc, char const *argv[]) {
  cout << "Got " << argc << " params\n";
  string program_name = argv[0];
  cout << "Progam: " << program_name << endl;
  for (int i = 1; i < argc; ++i) {
    cout << "Param: " << argv[i] << endl;
  }

}
