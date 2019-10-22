#include <iostream>
#include <thread>
#include <chrono>
#include <vector>
#include <atomic>
#include <mutex>
#include <future>

int square(int x) {
  //std::cout << "thread " << std::this_thread::get_id() << " started" << std::endl;
  std::this_thread::sleep_for(std::chrono::seconds(10));
  return x * x;
}

int main() {
  std::vector<std::future<int>> tasks {};
  for (int i = 1; i <= 100; ++i) {
    tasks.push_back(std::async(std::launch::async, &square, i));
  }

  int out = 0;
  for (auto& task: tasks) {
    int v = task.get();
    out += v;
  }
  std::cout << "the thread got: " << out << std::endl;
}
