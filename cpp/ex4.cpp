#include <deque>
#include <mutex>
#include <future>
#include <thread>
#include <utility>

std::mutex mut;
std::deque< std::packaged_task<void()> > tasks;
bool gui_shutdown_message_received();
void get_and_process_gui_message();

void gui_thread() {
  while (!gui_shutdown_message_received()) {
    get_and_process_gui_message();
    std::packaged_task<void()> task;
    {
      std::lock_guard<std::mutex> lock{mut};
      if (tasks.empty()) continue;
      task = std::move(tasks.front());
      tasks.pop_front();
    }
  }
}

std::thread gui_bg_thread(gui_thread);

template<typename Func>
std::future<void> post_task_for_gui_thread(Func f) {}
