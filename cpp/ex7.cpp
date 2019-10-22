#include <iostream>
#include <thread>
#include <functional>
#include <mutex>
#include <condition_variable>
#include <chrono>

class Application {
  private:
    std::mutex m_mutex;
    std::condition_variable m_cond;
    bool m_bDataLoaded;

  public:
    Application() : m_bDataLoaded{false} {}

    void LoadData() {
      std::this_thread::sleep_for(std::chrono::milliseconds(10000));
      std::cout << "Loading data from xml." << std::endl;
      std::lock_guard<std::mutex> lock{m_mutex};
      m_bDataLoaded = true;
      m_cond.notify_one();
    }

    bool isDataLoaded() {
      return m_bDataLoaded;
    }

    void mainTask() {

      std::cout << "Do some handshaking." << std::endl;
      std::unique_lock<std::mutex> lock{m_mutex};
      m_cond.wait(lock, std::bind(&Application::isDataLoaded, this));
      std::cout << "Do processing on loaded data." << std::endl;
    }
};


int main() {
  Application app;
  std::thread thread1(&Application::mainTask, &app);
  std::thread thread2(&Application::LoadData, &app);

  thread2.join();
  thread1.join();
  return 0;
}
