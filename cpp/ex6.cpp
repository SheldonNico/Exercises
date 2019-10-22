#include <iostream>
#include <thread>
#include <mutex>

class Application {
  private:
    std::mutex m_mutex;
    bool m_bDataLoaded;

  public:
    Application() : m_bDataLoaded{false} {};

    void LoadData() {
      // sleep 2 second
      std::this_thread::sleep_for(std::chrono::milliseconds(10000));
      std::cout << "Loading Data from XML" << std::endl;


      std::lock_guard<std::mutex> lock{m_mutex};
      m_bDataLoaded = true;
    }

    void main() {
      std::cout << "Do some handshaking" << std::endl;
      // use lock and looop check m_bDataLoaded
      m_mutex.lock();
      while (m_bDataLoaded != true) {
        std::cout << "check load flag!" << std::endl;
        m_mutex.unlock();
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        m_mutex.lock();
      }
      m_mutex.unlock();

      std::cout << "do pressing on loaded data" << std::endl;
    }
};

int main() {
  Application app;

  std::thread thread1(&Application::main, &app);
  std::thread thread2(&Application::LoadData, &app);

  thread2.join();
  thread1.join();

  return 0;
}
